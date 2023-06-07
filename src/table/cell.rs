use crate::table::{Align, Content, Overflow, Padding};

#[derive(Debug, Clone, Default)]
pub struct Cell {
    content: Content,
    overflow: Overflow,
    width: Option<usize>,
    align: Align,
    padding: Padding,
    merge: Option<usize>,
}

impl Cell {
    pub fn with_content(mut self, content: Content) -> Self {
        self.content = content;
        self
    }
    pub fn with_overflow(mut self, overflow: Overflow) -> Self {
        self.overflow = overflow;
        self
    }
    pub fn with_width(mut self, width: Option<usize>) -> Self {
        self.width = width;
        self
    }
    pub fn with_align(mut self, align: Align) -> Self {
        self.align = align;
        self
    }
    pub fn with_padding(mut self, padding: Padding) -> Self {
        self.padding = padding;
        self
    }
    pub fn with_merge(mut self, merge: Option<usize>) -> Self {
        self.merge = merge;
        self
    }

    pub fn set_content(&mut self, content: Content) {
        self.content = content
    }
    pub fn set_overflow(&mut self, overflow: Overflow) {
        self.overflow = overflow;
    }
    pub fn set_width(&mut self, width: Option<usize>) {
        self.width = width;
    }
    pub fn set_align(&mut self, align: Align) {
        self.align = align;
    }
    pub fn set_padding(&mut self, padding: Padding) {
        self.padding = padding;
    }
    pub fn set_merge(&mut self, merge: Option<usize>) {
        self.merge = merge;
    }
}
