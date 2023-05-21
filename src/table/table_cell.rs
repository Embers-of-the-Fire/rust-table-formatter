use colored::Colorize;

use super::{CellOverflow, FORMATTER};

pub struct TableCell {
    cell: Cell,
    position: CellPosition,
    overflow: CellOverflow,
    width: Option<usize>,
    formatter: Vec<FORMATTER>,
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
        }
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

    pub fn clone_text(&self) -> Self {
        Self {
            cell: self.cell.clone(),
            overflow: self.overflow.clone(),
            width: self.width.clone(),
            formatter: vec![],
            position: self.position.clone(),
        }
    }

    pub fn render(&self, width: usize) -> String {
        let (content, content_length) = self.cell.render_with_length(width, self.overflow);
        let content = self
            .formatter
            .iter()
            .fold(content.normal(), |acc, x| x(acc));
        if self.cell.is_splitter() {
            format!("─{}─", content)
        } else {
            match self.position {
                CellPosition::Left => {
                    format!(" {}{} ", content, " ".repeat(width - content_length))
                }
                CellPosition::Right => {
                    format!(" {}{} ", " ".repeat(width - content_length), content)
                }
                CellPosition::Middle => {
                    let diff = width - content_length;
                    if content_length == width {
                        format!(" {} ", content)
                    } else if (diff & 1) == 1 {
                        let left = (diff + 1) / 2;
                        let right = diff - left;
                        format!(" {}{}{} ", " ".repeat(left), content, " ".repeat(right))
                    } else {
                        let d = diff / 2;
                        format!(" {}{}{} ", " ".repeat(d), content, " ".repeat(d))
                    }
                }
            }
        }
    }
}

#[test]
fn test_render_tablecell() {
    let cell = TableCell::new(Cell::TextCell("123123".into()))
        .with_position(CellPosition::Middle)
        .with_formatter(vec![|v| Colorize::yellow(v), |v| Colorize::bold(v)]);
    let render_res = cell.render(5);
    println!("render result:\n----\n{}\n----", render_res);
    let cell = TableCell::new(Cell::Splitter);
    let render_res = cell.render(5);
    println!("render result:\n----\n{}\n----", render_res);
}

#[derive(Debug, Clone)]
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
                let mut string = String::new();
                let mut now_index = 0;
                let mut cache_string = String::new();
                let mut flag = false;
                let mut str_len: usize = 0;
                for c in text.chars() {
                    if now_index < width - 3 {
                        string += c.to_string().as_str();
                    } else if now_index < width {
                        cache_string += c.to_string().as_str();
                        if str_len == 0 {
                            str_len = now_index;
                        }
                    } else {
                        flag = true;
                        break;
                    }
                    now_index += 1;
                }
                if now_index < width {
                    str_len = now_index;
                }
                if !flag {
                    (
                        string + cache_string.as_str(),
                        str_len + cache_string.chars().fold(0_usize, |acc, _| acc + 1),
                    )
                } else {
                    (
                        match overflow {
                            CellOverflow::Hide => string + cache_string.as_str(),
                            CellOverflow::Ellipsis => string + "...",
                        },
                        width,
                    )
                }
            }
        }
    }
}

#[test]
fn test_render_cell() {
    let cell = Cell::TextCell("135123234".into());
    let render_res = cell.render_with_length(15, CellOverflow::Ellipsis);
    println!("{:#?}", render_res);
}
