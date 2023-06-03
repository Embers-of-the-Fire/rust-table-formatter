#[derive(Debug, Clone, Copy, Default)]
pub struct CellPadding {
    left: usize,
    right: usize,
}

impl CellPadding {
    pub const NONE: CellPadding = CellPadding { left: 0, right: 0 };

    pub const fn new(left: usize, right: usize) -> Self {
        Self { left, right }
    }

    pub const fn set_left(mut self, left: usize) -> Self {
        self.left = left;
        self
    }
    pub const fn set_right(mut self, right: usize) -> Self {
        self.right = right;
        self
    }

    pub const fn left(&self) -> usize {
        self.left
    }
    pub const fn right(&self) -> usize {
        self.right
    }
}
