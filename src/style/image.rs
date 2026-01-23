#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum ImageFit {
    #[default]
    Contain,
    Cover,
    Fill,
    None,
}
