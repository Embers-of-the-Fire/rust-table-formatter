use colored::Colorize;
use itertools::Itertools;

use super::{CellOverflow, CellPadding, FORMATTER};

pub struct TableCell {
    cell: Cell,
    position: CellPosition,
    overflow: CellOverflow,
    padding: CellPadding,
    width: Option<usize>,
    formatter: Vec<FORMATTER>,
}

impl<T: Into<Cell>> From<T> for TableCell {
    fn from(value: T) -> Self {
        TableCell::new(value.into())
    }
}

impl TableCell {
    pub fn new(cell: Cell) -> TableCell {
        let width = cell.get_width();
        Self {
            cell,
            position: CellPosition::Left,
            overflow: CellOverflow::Ellipsis,
            width,
            formatter: vec![],
            padding: CellPadding::NONE,
        }
    }
    pub fn from_vec<T: Into<TableCell>>(v: Vec<T>) -> Vec<TableCell> {
        v.into_iter().map(|i| i.into()).collect_vec()
    }
    pub fn with_padding(mut self, padding: CellPadding) -> Self {
        self.padding = padding;
        self
    }
    pub fn set_padding(&mut self, padding: CellPadding) {
        self.padding = padding;
    }
    pub fn with_position(mut self, position: CellPosition) -> Self {
        self.position = position;
        self
    }
    pub fn with_overflow(mut self, overflow: CellOverflow) -> Self {
        self.overflow = overflow;
        self
    }
    pub fn set_position(&mut self, position: CellPosition) {
        self.position = position;
    }
    pub fn set_overflow(&mut self, overflow: CellOverflow) {
        self.overflow = overflow;
    }
    pub fn with_width(mut self, width: usize) -> Self {
        self.width = Some(width);
        self
    }
    pub fn set_width(&mut self, width: usize) {
        self.width = Some(width);
    }
    pub fn get_width(&self) -> Option<usize> {
        self.width
    }
    pub fn with_formatter(mut self, mut formatter: Vec<FORMATTER>) -> Self {
        self.formatter.append(&mut formatter);
        self
    }
    pub fn set_formatter(&mut self, mut formatter: Vec<FORMATTER>) {
        self.formatter.append(&mut formatter);
    }

    pub fn clone_cell(&self) -> Self {
        Self {
            cell: self.cell.clone(),
            overflow: self.overflow,
            width: self.width,
            formatter: vec![],
            position: self.position,
            padding: self.padding,
        }
    }

    pub fn render(&self, width: usize) -> String {
        if self.padding.left() + self.padding.right() >= width {
            " ".repeat(width + 2)
        } else {
            let cell_content_width = width - self.padding.left() - self.padding.right();
            let (content, content_length) = self
                .cell
                .render_with_length(cell_content_width, self.overflow);
            let content = self
                .formatter
                .iter()
                .fold(content.normal(), |acc, x| x(acc));
            if self.cell.is_splitter() {
                format!("─{}─", content)
            } else {
                let content = match self.position {
                    CellPosition::Left => {
                        format!(
                            "{}{}",
                            content,
                            " ".repeat(cell_content_width - content_length),
                        )
                    }
                    CellPosition::Right => {
                        format!(
                            "{}{}",
                            " ".repeat(cell_content_width - content_length),
                            content,
                        )
                    }
                    CellPosition::Middle => {
                        let diff = cell_content_width - content_length;
                        if content_length == cell_content_width {
                            format!("{}", content)
                        } else if (diff & 1) == 1 {
                            let left = (diff + 1) / 2;
                            let right = diff - left;
                            format!("{}{}{}", " ".repeat(left), content, " ".repeat(right))
                        } else {
                            let d = diff / 2;
                            format!("{}{}{}", " ".repeat(d), content, " ".repeat(d))
                        }
                    }
                };
                format!(
                    " {}{}{} ",
                    " ".repeat(self.padding.left()),
                    content,
                    " ".repeat(self.padding.right())
                )
            }
        }
    }

    pub fn render_raw(&self, width: usize) -> String {
        if self.padding.left() + self.padding.right() >= width {
            " ".repeat(width + 2)
        } else {
            let cell_content_width = width - self.padding.left() - self.padding.right();
            let (content, content_length) = self
                .cell
                .render_with_length(cell_content_width, self.overflow);
            if self.cell.is_splitter() {
                format!("─{}─", content)
            } else {
                let content = match self.position {
                    CellPosition::Left => {
                        format!(
                            "{}{}",
                            content,
                            " ".repeat(cell_content_width - content_length),
                        )
                    }
                    CellPosition::Right => {
                        format!(
                            "{}{}",
                            " ".repeat(cell_content_width - content_length),
                            content,
                        )
                    }
                    CellPosition::Middle => {
                        let diff = cell_content_width - content_length;
                        if content_length == cell_content_width {
                            content
                        } else if (diff & 1) == 1 {
                            let left = (diff + 1) / 2;
                            let right = diff - left;
                            format!("{}{}{}", " ".repeat(left), content, " ".repeat(right))
                        } else {
                            let d = diff / 2;
                            format!("{}{}{}", " ".repeat(d), content, " ".repeat(d))
                        }
                    }
                };
                format!(
                    " {}{}{} ",
                    " ".repeat(self.padding.left()),
                    content,
                    " ".repeat(self.padding.right())
                )
            }
        }
    }
}

#[test]
fn test_render_tablecell() {
    let cell = TableCell::new(Cell::TextCell("123123".into()))
        .with_position(CellPosition::Middle)
        .with_padding(CellPadding::new(0, 0));
    let render_res = cell.render(5);
    println!("render result:\n----\n{}\n----", render_res);
    let cell = TableCell::new(Cell::TextCell("123123".into()))
        .with_position(CellPosition::Middle)
        .with_padding(CellPadding::new(1, 0));
    let render_res = cell.render(5);
    println!("render result:\n----\n{}\n----", render_res);
    let cell = TableCell::new(Cell::TextCell("123123".into()))
        .with_position(CellPosition::Middle)
        .with_padding(CellPadding::new(1, 1));
    let render_res = cell.render(5);
    println!("render result:\n----\n{}\n----", render_res);
    let cell = TableCell::new(Cell::Splitter);
    let render_res = cell.render(5);
    println!("render result:\n----\n{}\n----", render_res);
}

#[derive(Debug, Clone, Copy)]
pub enum CellPosition {
    Left,
    Middle,
    Right,
}

#[derive(Debug, Clone)]
pub enum Cell {
    TextCell(String),
    Splitter,
}

impl<T: ToString> From<Option<T>> for Cell {
    fn from(value: Option<T>) -> Self {
        if let Some(v) = value {
            Cell::TextCell(v.to_string())
        } else {
            Cell::Splitter
        }
    }
}

impl Cell {
    pub fn is_splitter(&self) -> bool {
        matches!(self, Cell::Splitter)
    }

    pub fn get_width(&self) -> Option<usize> {
        match self {
            Cell::TextCell(text) => Some(text.chars().map(|_| 1_usize).sum::<usize>()),
            Cell::Splitter => None,
        }
    }

    pub fn get_string(&self) -> Option<&String> {
        match self {
            Cell::TextCell(text) => Some(text),
            Cell::Splitter => None,
        }
    }

    fn render_with_length(&self, width: usize, overflow: CellOverflow) -> (String, usize) {
        match self {
            Cell::Splitter => ("─".repeat(width), width),
            Cell::TextCell(text) => {
                if width <= 3 {
                    let mut tc = text.chars();
                    let mut string = String::new();
                    for _ in 0..(width - 1) {
                        if let Some(c) = tc.next() {
                            string.push(c);
                        } else {
                            break;
                        }
                    }
                    match tc.next() {
                        None => {
                            let length = string.chars().count();
                            (string, length)
                        }
                        Some(c) => match tc.next() {
                            None => {
                                string.push(c);
                                (string, width)
                            }
                            Some(_) => match overflow {
                                CellOverflow::Ellipsis => (string + ".", width),
                                CellOverflow::Hide => {
                                    string.push(c);
                                    (string, width)
                                }
                            },
                        },
                    }
                } else {
                    let mut show_string = String::new();
                    let mut tc = text.chars();
                    let mut length = 0;
                    for _ in 0..(width - 3) {
                        if let Some(c) = tc.next() {
                            show_string.push(c);
                            length += 1;
                        } else {
                            break;
                        }
                    }
                    match tc.next() {
                        None => (show_string, length),
                        Some(c) => {
                            let mut res_string = String::new();
                            res_string.push(c);
                            let mut innl = 1;
                            for _ in 0..2 {
                                if let Some(ic) = tc.next() {
                                    res_string.push(ic);
                                    innl += 1;
                                } else {
                                    break;
                                }
                            }
                            if tc.next().is_some() {
                                match overflow {
                                    CellOverflow::Ellipsis => (show_string + "...", width),
                                    CellOverflow::Hide => {
                                        (show_string + res_string.as_str(), width)
                                    }
                                }
                            } else {
                                (show_string + res_string.as_str(), length + innl)
                            }
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn test_render_cell() {
    let cell = Cell::TextCell("12123123343🍇".into());
    let render_res = cell.render_with_length(7, CellOverflow::Hide);
    println!("{:#?}", render_res);
}
