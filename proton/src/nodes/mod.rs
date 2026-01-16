mod image;
mod text;
mod view;

pub use image::{Image, ImageNode, ImageSource};
pub use text::{Text, TextNode};
pub use view::{View, ViewNode};

#[derive(Debug, Clone)]
pub enum Node {
    View(ViewNode),
    Text(TextNode),
    Image(ImageNode),
}
