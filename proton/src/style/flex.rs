#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum FlexDirection {
    Row,
    #[default]
    Column,
}

// main axis alignment
// if column -> y axis
// if row -> x axis
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Justify {
    #[default]
    Start,
    End,
    Center,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

// cross axis alignment
// if column -> x axis
// if row -> y axis
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Align {
    Start,
    End,
    Center,
    #[default]
    Stretch,
}
