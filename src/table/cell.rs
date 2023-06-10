use colored::ColoredString;

use crate::table::{Align, Content, Overflow, Padding};

use super::FormatterFunc;

#[derive(Clone, Default)]
pub struct Cell {
    content: Content,
    overflow: Overflow,
    width: Option<usize>,
    align: Align,
    padding: Padding,
    merge: Option<usize>,
    formatter: Vec<FormatterFunc>,
}

impl Cell {
    pub fn with_content(mut self, content: Content) -> Self {
        self.width = content.get_width();
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
    pub fn with_formatter(mut self, formatter: Vec<FormatterFunc>) -> Self {
        self.formatter = formatter;
        self
    }

    pub fn set_content(&mut self, content: Content) {
        self.width = content.get_width();
        self.content = content;
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
    pub fn set_formatter(mut self, formatter: Vec<FormatterFunc>) {
        self.formatter = formatter;
    }

    pub fn append_formatter(&mut self, formatter: &mut Vec<FormatterFunc>) {
        self.formatter.append(formatter);
    }
    pub fn with_appended_formatter(mut self, formatter: &mut Vec<FormatterFunc>) -> Self {
        self.formatter.append(formatter);
        self
    }

    pub fn with_span(mut self, span: usize) -> Vec<Self> {
        self.set_merge(Some(span));
        self.set_width(None);
        let mut v = vec![self];
        v.extend(std::iter::repeat(Self::default()).take(span));
        v
    }

    pub fn get_width(&self) -> Option<usize> {
        self.width
    }
    pub fn get_merge(&self) -> Option<usize> {
        self.merge
    }
    pub fn get_content(&self) -> &Content {
        &self.content
    }
    pub fn get_align(&self) -> Align {
        self.align
    }

    pub(crate) fn render_with_width_raw(&self, width: usize) -> String {
        if self.content.have_default_padding() {
            let width = width - self.padding.left - self.padding.right;
            let (content, w) = self.content.render_with_width(width, self.overflow);
            let content = match self.align {
                Align::Left => format!("{}{}", content, " ".repeat(width - w)),
                Align::Right => format!("{}{}", " ".repeat(width - w), content),
                Align::Center => {
                    let left = (width - w) / 2;
                    let right = width - w - left;
                    format!("{}{}{}", " ".repeat(left), content, " ".repeat(right))
                }
            };
            format!(
                " {}{}{} ",
                " ".repeat(self.padding.left),
                content,
                " ".repeat(self.padding.right)
            )
        } else {
            let (content, _) = self.content.render_with_width(width + 2, self.overflow);
            content
        }
    }

    pub(crate) fn render_with_width(&self, width: usize) -> ColoredString {
        let result = self.render_with_width_raw(width);
        self.formatter
            .iter()
            .fold(ColoredString::from(result.as_str()), |acc, func| {
                func.run(acc)
            })
    }
}

#[test]
fn test_render_cell() {
    let cell = Cell::default()
        .with_content(Content::new("123"))
        .with_overflow(Overflow::Ellipsis);
    let rendered = cell.render_with_width(2);
    assert_eq!(" 1. ", format!("{}", rendered));

    let cell = Cell::default()
        .with_content(Content::new("123123"))
        .with_overflow(Overflow::Ellipsis);
    let rendered = cell.render_with_width(4);
    assert_eq!(" 1... ", format!("{}", rendered));

    let cell = Cell::default()
        .with_content(Content::new("123123"))
        .with_overflow(Overflow::Ellipsis);
    let rendered = cell.render_with_width(6);
    assert_eq!(" 123123 ", format!("{}", rendered));

    let cell = Cell::default()
        .with_content(Content::new("123123"))
        .with_overflow(Overflow::Hidden);
    let rendered = cell.render_with_width(5);
    assert_eq!(" 12312 ", format!("{}", rendered));

    let cell = Cell::default()
        .with_content(Content::new("123123"))
        .with_overflow(Overflow::Ellipsis)
        .with_padding(Padding::new(1, 1));
    let rendered = cell.render_with_width(6);
    assert_eq!("  1...  ", format!("{}", rendered));

    let cell = Cell::default()
        .with_content(Content::new("123123"))
        .with_overflow(Overflow::Ellipsis)
        .with_padding(Padding::new(1, 3));
    let rendered = cell.render_with_width(6);
    assert_eq!("  1.    ", format!("{}", rendered));

    let cell = Cell::default()
        .with_content(Content::new("123123"))
        .with_overflow(Overflow::Ellipsis)
        .with_align(Align::Center);
    let rendered = cell.render_with_width(10);
    assert_eq!("   123123   ", format!("{}", rendered));

    let cell = Cell::default()
        .with_content(Content::new("123123"))
        .with_overflow(Overflow::Ellipsis)
        .with_align(Align::Center);
    let rendered = cell.render_with_width(9);
    assert_eq!("  123123   ", format!("{}", rendered));

    let cell = Cell::default()
        .with_content(Content::new("123123"))
        .with_overflow(Overflow::Ellipsis)
        .with_align(Align::Right);
    let rendered = cell.render_with_width(8);
    assert_eq!("   123123 ", format!("{}", rendered));
}
