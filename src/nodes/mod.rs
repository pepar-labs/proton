mod image;
mod list_view;
mod scroll_view;
mod text;
mod view;

pub use image::{Image, ImageNode, ImageSource};
pub use list_view::{ListView, ListViewNode};
pub use scroll_view::{ScrollView, ScrollViewNode};
pub use text::{Text, TextNode};
pub use view::{View, ViewNode};

#[derive(Debug, Clone)]
pub enum Node {
    View(ViewNode),
    Text(TextNode),
    Image(ImageNode),
    ScrollView(ScrollViewNode),
    ListView(ListViewNode),
}
