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
    left: usize,
    right: usize,
}

impl Padding {
    pub const NONE: Padding = Padding::new(0, 0);
    pub const fn new(left: usize, right: usize) -> Self {
        Self { left, right }
    }
}
