#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Overflow {
    Ellipsis,
    Hidden,
}

impl Default for Overflow {
    fn default() -> Self {
        Self::Ellipsis
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Align {
    Left,
    Center,
    Right,
}

impl Default for Align {
    fn default() -> Self {
        Self::Left
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Padding {
    pub left: usize,
    pub right: usize,
}

impl Padding {
    pub const NONE: Padding = Padding::new(0, 0);
    pub const fn new(left: usize, right: usize) -> Self {
        Self { left, right }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Border {
    pub left: bool,
    pub right: bool,
    pub top: bool,
    pub bottom: bool,
}

impl Border {
    pub const NONE: Border = Border::new(false, false, false, false);
    pub const HORIZONTAL: Border = Border::new(false, false, true, true);
    pub const VERTICAL: Border = Border::new(true, true, false, false);
    pub const ALL: Border = Border::new(true, true, true, true);
    pub const fn new(left: bool, right: bool, top: bool, bottom: bool) -> Self {
        Self {
            left,
            right,
            top,
            bottom,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Renderer {
    Normal,
    Raw,
    Markdown,
}
