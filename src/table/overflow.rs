#[derive(Debug, Clone, Copy)]
pub enum CellOverflow {
    Hide,
    Ellipsis,
}

impl Default for CellOverflow {
    fn default() -> Self {
        Self::Ellipsis
    }
}