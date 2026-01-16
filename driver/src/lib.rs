use core::fmt;
use std::mem;
use std::time::Duration;

use bincode::Options;
use image::GenericImageView;
use rusb::{open_device_with_vid_pid, DeviceHandle, GlobalContext, Result};
use serde::{Deserialize, Serialize};

mod usb;

// to obtain in and out endpoint
// run lsusb -d vid:pid -v
// for this case vid = 0x048d and pid = 8951
const IN_EP: u8 = 0x81;
const OUT_EP: u8 = 0x02;

// given on the official waveshare website

// dir: bulk_in
// returns 40 bytes
// "Generic Storage RamDisc 1.00"
const SCSI_INQUIRY_CMD: [u8; 16] = [0x12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

// returns 112 bytes
/**
returning struct

 typedef struct _TRSP_SYSTEM_INFO_DATA
 {
 unsigned int uiStandardCmdNo; // Standard command number2T-con
 Communication Protocol
 unsigned int uiExtendCmdNo; // Extend command number unsigned int uiSignature; // 31 35 39 38h (8951)
 unsigned int uiVersion; // command table version
 unsigned int uiWidth; // Panel Width
 unsigned int uiHeight; // Panel Height
 unsigned int uiUpdateBufBase; // Update Buffer Address
 unsigned int uiImageBufBase; // Image Buffer Address(index 0)
 unsigned int uiTemperatureNo; // Temperature segment number
 unsigned int uiModeNo; // Display mode number
 unsigned int uiFrameCount[8]; // Frame count for each mode(8).
 unsigned int uiNumImgBuf; //Numbers of Image buffer
 unsigned int uiReserved[9]; // Don't care
 void* lpCmdInfoDatas[1]; // Command table pointer
 } TRSP_SYSTEM_INFO_DATA;
**/
// dir: bulk_in
const GET_SYS_CMD: [u8; 16] = [
    0xfe, 0, 0x38, 0x39, 0x35, 0x31, 0x80, 0, 0x01, 0, 0x02, 0, 0, 0, 0, 0,
];

// display area function
// dir: bulk_out
const DPY_AREA_CMD: [u8; 16] = [0xfe, 0, 0, 0, 0, 0, 0x94, 0, 0, 0, 0, 0, 0, 0, 0, 0];

// host load image area function
// dir: bulk_out
const LD_IMG_AREA_CMD: [u8; 16] = [0xfe, 0, 0, 0, 0, 0, 0xa2, 0, 0, 0, 0, 0, 0, 0, 0, 0];

const MAX_TRANSFER_SIZE: usize = 60 * 1024;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Rotation {
    Rotate0,
    Rotate90,
    Rotate180,
    Rotate270,
}

#[repr(C)]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub enum Mode {
    // clear the screen
    CLEAR = 0,
    // direct update, fast, 2 shades
    DU,
    // grayscale clearing, 16 shades, slow, best quality
    GC16,
    // grayscale, 16 shades, medium speed
    GL16,
    // grayscale with reduced flashing
    GLR16,
    // grayscale with direct update
    GLD16,
    // direct update with 4 shades
    DU4,
    // animation mode, very fast, 2 shades, ghosting
    A2,
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[repr(C)]
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct SystemInfo {
    pub std_cmd_no: u32,
    pub ext_cmd_no: u32,
    pub signature: u32,
    pub version: u32,
    pub width: u32,
    pub height: u32,
    pub update_buf_base: u32,
    pub img_buf_base: u32,
    pub temp_no: u32,
    pub mode_no: u32,
    pub frame_count: [u32; 8],
    pub num_img_buf: u32,
    reserved: [u32; 9],
}

#[repr(C)]
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Inquiry {
    ignore: [u8; 8],
    pub vendor_identification: [u8; 8],
    pub product_identification: [u8; 16],
    pub product_revision_level: [u8; 4],
    ignore_end: [u8; 4],
}

#[repr(C)]
#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Area {
    addr: u32,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

#[repr(C)]
#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct DisplayArea {
    addr: u32,
    disp_mode: Mode,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    wait_ready: u32,
}

fn open_device() -> Option<DeviceHandle<GlobalContext>> {
    // run lsusb to find the vid, pid
    open_device_with_vid_pid(0x048d, 0x8951)
}

pub struct Device {
    connection: usb::ScsiOverUsbConnection,
    system_info: Option<SystemInfo>,
    framebuffer: Vec<u8>,
    rotation: Rotation,
}

impl Drop for Device {
    fn drop(&mut self) {
        self.connection
            .device_handle
            .release_interface(0)
            .expect("failed to release interface connection");
    }
}

impl Device {
    pub fn connect() -> Result<Device> {
        let timeout = Duration::from_millis(2000);
        let device_handle = open_device().expect("cannot open the device");
        if let Err(e) = device_handle.set_auto_detach_kernel_driver(true) {
            println!(
                "platform doesn't support auto detach terminal, {:?}",
                e.to_string()
            );
        }
        device_handle
            .claim_interface(0)
            .expect("failed to claim device interface");

        let mut result = Device {
            connection: usb::ScsiOverUsbConnection {
                device_handle,
                endpoint_out: OUT_EP,
                endpoint_in: IN_EP,
                timeout,
            },
            system_info: None,
            framebuffer: Vec::new(),
            rotation: Rotation::Rotate0,
        };
        let system_info = result.get_sys()?;
        let width = system_info.width as usize;
        let height = system_info.height as usize;
        result.framebuffer = vec![0; width * height];
        result.system_info = Some(system_info);
        Ok(result)
    }

    pub fn inquiry(&mut self) -> Result<Inquiry> {
        let inq: Inquiry = self
            .connection
            .read_command(&SCSI_INQUIRY_CMD, bincode::options())?;
        Ok(inq)
    }

    pub fn get_sys(&mut self) -> Result<SystemInfo> {
        self.connection
            .read_command(&GET_SYS_CMD, bincode::options().with_big_endian())
    }

    pub fn get_system_info(&self) -> Option<&SystemInfo> {
        self.system_info.as_ref()
    }

    fn ld_image_area(&mut self, area: Area, data: &[u8]) -> Result<()> {
        self.connection.write_command(
            &LD_IMG_AREA_CMD,
            area,
            data,
            bincode::options().with_big_endian(),
        )
    }

    fn dpy_area(&mut self, display_area: DisplayArea) -> Result<()> {
        self.connection.write_command(
            &DPY_AREA_CMD,
            display_area,
            &[],
            bincode::options().with_big_endian(),
        )
    }

    pub fn update_region(
        &mut self,
        image: &image::DynamicImage,
        x: u32,
        y: u32,
        mode: Mode,
    ) -> Result<()> {
        let data = image.as_bytes();
        let (w, h) = image.dimensions();
        let width: usize = w as usize;
        let height: usize = h as usize;

        // we can only send the MAX_TRANSFER_SIZE, so we send multiple chunks
        let size = width * height;
        let mut i: usize = 0;
        let mut row_height = (MAX_TRANSFER_SIZE - mem::size_of::<Area>()) / width;

        let address = self.get_system_info().unwrap().img_buf_base;
        while i < size {
            if (i / width) + row_height > height {
                row_height = height - (i / width);
            }
            self.ld_image_area(
                Area {
                    addr: address,
                    x,
                    y: y + (i / width) as u32,
                    width: w,
                    height: row_height as u32,
                },
                &data[i..i + width * row_height],
            )?;
            i += row_height * width;
        }
        self.dpy_area(DisplayArea {
            addr: address,
            disp_mode: mode,
            x,
            y,
            width: w,
            height: h,
            wait_ready: 1,
        })?;

        Ok(())
    }

    pub fn flush_region(&mut self, x: u32, y: u32, w: u32, h: u32, mode: Mode) -> Result<()> {
        let system_info = self.get_system_info().unwrap();
        let hw_width = system_info.width as usize;
        let hw_height = system_info.height as usize;

        match self.rotation {
            Rotation::Rotate0 => {
                let mut region_data = Vec::new();
                for row in y..(y + h) {
                    let start = (row as usize * hw_width) + x as usize;
                    let end = start + w as usize;
                    region_data.extend_from_slice(&self.framebuffer[start..end]);
                }
                let img = image::GrayImage::from_vec(w, h, region_data)
                    .expect("can't create image from framebuffer");
                let dynamic_img = image::DynamicImage::ImageLuma8(img);
                self.update_region(&dynamic_img, x, y, mode)
            }
            _ => {
                // when rotation is not 0, flush the entire framebuffer
                let img = image::GrayImage::from_vec(
                    hw_width as u32,
                    hw_height as u32,
                    self.framebuffer.clone(),
                )
                .expect("can't create image from framebuffer, rotation mode");
                let dynamic_img = image::DynamicImage::ImageLuma8(img);
                self.update_region(&dynamic_img, 0, 0, mode)
            }
        }
    }

    pub fn clear_framebuffer(&mut self) {
        self.framebuffer.fill(255);
    }

    pub fn dimensions(&self) -> (u32, u32) {
        if let Some(info) = &self.system_info {
            match self.rotation {
                Rotation::Rotate0 | Rotation::Rotate180 => (info.width, info.height),
                Rotation::Rotate90 | Rotation::Rotate270 => (info.height, info.width),
            }
        } else {
            (0, 0)
        }
    }

    pub fn set_rotation(&mut self, rotation: Rotation) {
        self.rotation = rotation;
    }

    pub fn rotation(&self) -> Rotation {
        self.rotation
    }

    pub fn flush(&mut self, mode: Mode) -> Result<()> {
        let (width, height) = self.dimensions();
        self.flush_region(0, 0, width, height, mode)
    }

    pub fn set_pixel(&mut self, x: i32, y: i32, color: u8) {
        if let Some(idx) = self.pixel_index(x, y) {
            self.framebuffer[idx] = color;
        }
    }

    pub fn get_pixel(&self, x: i32, y: i32) -> u8 {
        if let Some(idx) = self.pixel_index(x, y) {
            self.framebuffer[idx]
        } else {
            255
        }
    }

    fn pixel_index(&self, x: i32, y: i32) -> Option<usize> {
        let sys_info = self.system_info.as_ref()?;

        let hw_width = sys_info.width as i32;
        let hw_height = sys_info.height as i32;

        let (rot_x, rot_y) = match self.rotation {
            Rotation::Rotate0 => (x, y),
            Rotation::Rotate90 => (hw_width - 1 - y, x),
            Rotation::Rotate180 => (hw_width - 1 - x, hw_height - 1 - y),
            Rotation::Rotate270 => (y, hw_height - 1 - x),
        };

        if rot_x >= 0 && rot_x < hw_width && rot_y >= 0 && rot_y < hw_height {
            let idx = (rot_y as usize * hw_width as usize) + rot_x as usize;
            if idx < self.framebuffer.len() {
                return Some(idx);
            }
        }
        None
    }
}
