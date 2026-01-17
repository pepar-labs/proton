use image::{GrayImage, Luma};

use crate::font::Fonts;
use crate::layout::{LayoutNode, LayoutTree, NodeData};
use crate::nodes::{Node, ScrollViewNode, ViewNode};
use crate::style::{Rect, Size};

use super::image::render_image;
use super::primitives::fill_rect_clipped;
use super::target::RenderTarget;
use super::text::render_text;

pub struct Renderer {
    fonts: Fonts,
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            fonts: Fonts::new(),
        }
    }

    pub fn render(&self, layout: &LayoutTree, root: &Node, size: Size) -> GrayImage {
        let mut image = GrayImage::from_pixel(size.width as u32, size.height as u32, Luma([255u8]));
        self.render_to(&mut image, layout, root);
        image
    }

    pub fn render_to<T: RenderTarget>(&self, target: &mut T, layout: &LayoutTree, root: &Node) {
        self.render_node(target, root, layout, 0);
    }

    fn render_node<T: RenderTarget>(
        &self,
        target: &mut T,
        node: &Node,
        layout: &LayoutTree,
        index: usize,
    ) -> usize {
        let layout_node = &layout.nodes[index];

        match node {
            Node::View(view) => self.render_view(target, view, layout_node, layout, index, None),
            Node::Text(text) => {
                render_text(target, &self.fonts, text, layout_node, None);
                index + 1
            }
            Node::Image(img) => {
                render_image(target, img, layout_node, None);
                index + 1
            }
            Node::ScrollView(scroll) => {
                self.render_scroll_view(target, scroll, layout_node, layout, index)
            }
        }
    }

    fn render_view<T: RenderTarget>(
        &self,
        target: &mut T,
        view: &ViewNode,
        layout_node: &LayoutNode,
        layout: &LayoutTree,
        index: usize,
        clip: Option<&Rect>,
    ) -> usize {
        let rect = &layout_node.rect;

        if let Some(color) = view.background {
            fill_rect_clipped(target, rect.x, rect.y, rect.width, rect.height, color, clip);
        }

        let mut next_index = index + 1;
        for child in &view.children {
            next_index = self.render_node_clipped(target, child, layout, next_index, clip);
        }

        next_index
    }

    fn render_scroll_view<T: RenderTarget>(
        &self,
        target: &mut T,
        scroll: &ScrollViewNode,
        layout_node: &LayoutNode,
        layout: &LayoutTree,
        index: usize,
    ) -> usize {
        let rect = &layout_node.rect;

        // Get scroll offset from NodeData
        let scroll_offset =
            if let Some(NodeData::ScrollView { scroll_offset, .. }) = &layout_node.data {
                *scroll_offset
            } else {
                scroll.scroll_offset
            };

        // Draw background if present
        if let Some(color) = scroll.background {
            fill_rect_clipped(target, rect.x, rect.y, rect.width, rect.height, color, None);
        }

        // Create clip rect for this scroll view (the viewport)
        let clip_rect = Rect {
            x: rect.x,
            y: rect.y,
            width: rect.width,
            height: rect.height,
        };

        // Render children with scroll offset applied
        let mut next_index = index + 1;
        for child in &scroll.children {
            next_index = self.render_node_scrolled(
                target,
                child,
                layout,
                next_index,
                &clip_rect,
                scroll_offset,
            );
        }

        next_index
    }

    fn render_node_clipped<T: RenderTarget>(
        &self,
        target: &mut T,
        node: &Node,
        layout: &LayoutTree,
        index: usize,
        clip: Option<&Rect>,
    ) -> usize {
        let layout_node = &layout.nodes[index];

        match node {
            Node::View(view) => self.render_view(target, view, layout_node, layout, index, clip),
            Node::Text(text) => {
                render_text(target, &self.fonts, text, layout_node, clip);
                index + 1
            }
            Node::Image(img) => {
                render_image(target, img, layout_node, clip);
                index + 1
            }
            Node::ScrollView(scroll) => {
                self.render_scroll_view(target, scroll, layout_node, layout, index)
            }
        }
    }

    fn render_node_scrolled<T: RenderTarget>(
        &self,
        target: &mut T,
        node: &Node,
        layout: &LayoutTree,
        index: usize,
        clip: &Rect,
        scroll_offset: f32,
    ) -> usize {
        let layout_node = &layout.nodes[index];

        // Create an adjusted layout node with scroll offset applied
        let adjusted_rect = Rect {
            x: layout_node.rect.x,
            y: layout_node.rect.y - scroll_offset,
            width: layout_node.rect.width,
            height: layout_node.rect.height,
        };
        let adjusted_layout_node = LayoutNode {
            rect: adjusted_rect,
            data: layout_node.data.clone(),
        };

        match node {
            Node::View(view) => self.render_view_scrolled(
                target,
                view,
                &adjusted_layout_node,
                layout,
                index,
                clip,
                scroll_offset,
            ),
            Node::Text(text) => {
                render_text(target, &self.fonts, text, &adjusted_layout_node, Some(clip));
                index + 1
            }
            Node::Image(img) => {
                render_image(target, img, &adjusted_layout_node, Some(clip));
                index + 1
            }
            Node::ScrollView(scroll) => {
                // Nested scroll views - just render normally for now
                self.render_scroll_view(target, scroll, &adjusted_layout_node, layout, index)
            }
        }
    }

    fn render_view_scrolled<T: RenderTarget>(
        &self,
        target: &mut T,
        view: &ViewNode,
        layout_node: &LayoutNode,
        layout: &LayoutTree,
        index: usize,
        clip: &Rect,
        scroll_offset: f32,
    ) -> usize {
        let rect = &layout_node.rect;

        if let Some(color) = view.background {
            fill_rect_clipped(
                target,
                rect.x,
                rect.y,
                rect.width,
                rect.height,
                color,
                Some(clip),
            );
        }

        let mut next_index = index + 1;
        for child in &view.children {
            next_index =
                self.render_node_scrolled(target, child, layout, next_index, clip, scroll_offset);
        }

        next_index
    }
}

impl Default for Renderer {
    fn default() -> Self {
        Self::new()
    }
}
