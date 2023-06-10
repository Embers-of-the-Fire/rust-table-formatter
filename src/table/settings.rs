/// Overflow setting for cells.
/// 
/// `Overflow::Ellipsis`: "hello world" -> "he..."
///
/// `Overflow::Hidden`: "hello" -> "hello"
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

/// Text alignment.
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

/// Padding around the content.
/// 
/// A string `"hello"` with `Padding{ left: 1, right: 1 }` will become `" ell "` but not `" hello "`.
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

/// Border of the table.
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

/// Render settings.
#[derive(Debug, Clone, Copy)]
pub enum Renderer {
    /// Render a normal table, with ansi color settings.
    Normal,
    /// Render a raw table, containing only contents.
    Raw,
    /// Render a markdown-formatted table. The alignment is determined by **the first row**, and the alignment of the rest of the table will be *ignored*.
    Markdown,
}
