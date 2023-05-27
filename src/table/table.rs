use colored::Colorize;
use itertools::Itertools;

use super::{Cell, CellOverflow, TableCell, TableColumn};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Border {
    pub top: bool,
    pub bottom: bool,
    pub left: bool,
    pub right: bool,
}

impl Border {
    pub const ALL: Border = Border {
        left: true,
        right: true,
        top: true,
        bottom: true,
    };
    pub const NONE: Border = Border {
        top: false,
        bottom: false,
        left: false,
        right: false,
    };
    pub const LEFT: Border = Border {
        left: true,
        right: false,
        top: false,
        bottom: false,
    };
    pub const RIGHT: Border = Border {
        left: false,
        right: true,
        top: false,
        bottom: false,
    };
    pub const TOP: Border = Border {
        left: false,
        right: false,
        top: true,
        bottom: false,
    };
    pub const BOTTOM: Border = Border {
        left: false,
        right: false,
        top: false,
        bottom: true,
    };

    pub const fn mixin(mut self, other: &Self) -> Self {
        if other.left {
            self.left = true
        }
        if other.right {
            self.right = true
        }
        if other.top {
            self.top = true
        }
        if other.bottom {
            self.bottom = true
        }
        self
    }

    pub const fn default() -> Self {
        Self {
            top: false,
            bottom: false,
            left: false,
            right: false,
        }
    }
}

pub struct Table {
    columns: Vec<TableColumn>,
    border: Border,
}

impl Table {
    pub fn new(columns: Vec<TableColumn>) -> Table {
        Table {
            columns,
            border: Border::default(),
        }
    }
    pub fn with_border(mut self, border: Border) -> Self {
        self.border = border;
        self
    }
    pub fn mixin_border(mut self, border: &Border) -> Self {
        self.border = self.border.mixin(border);
        self
    }
    pub fn with_overflow(mut self, overflow: CellOverflow) -> Self {
        self.columns
            .iter_mut()
            .for_each(|c| c.set_overflow(overflow));
        self
    }
    pub fn set_overflow(&mut self, overflow: CellOverflow) {
        self.columns
            .iter_mut()
            .for_each(|c| c.set_overflow(overflow));
    }

    pub fn from_cells(cells: Vec<Vec<TableCell>>) -> Table {
        Table::new(TableColumn::from_cells(cells))
    }

    pub fn from_data(mut title: Vec<TableCell>, mut data: Vec<Vec<TableCell>>) -> Table {
        let width = title
            .iter_mut()
            .map(|c| {
                c.set_formatter(vec![Colorize::bold]);
                1_usize
            })
            .sum::<usize>();
        let mut v = vec![title];
        v.push(
            (0..width)
                .map(|_| TableCell::new(Cell::Splitter))
                .collect_vec(),
        );
        v.append(&mut data);
        Table::from_cells(v)
    }

    pub fn render(&self) -> String {
        let csr: Vec<(Vec<String>, usize)> = self
            .columns
            .iter()
            .map(|c| (c.render(), c.get_render_width()))
            .collect_vec();
        let mut rows: Vec<Vec<String>> = Vec::new();
        let mut width = 0_usize;
        for (s, size) in csr {
            let mut pointer = 0_usize;
            for ss in s {
                if let Some(r) = rows.get_mut(pointer) {
                    r.push(ss);
                } else {
                    rows.push(vec![ss])
                }
                pointer += 1;
            }
            width += size;
        }
        width += 2;
        let content = rows
            .into_iter()
            .map(|row| {
                format!(
                    "{}{}{}",
                    if self.border.left { "|" } else { "" }.bold(),
                    row.join(""),
                    if self.border.left { "|" } else { "" }.bold()
                )
            })
            .join("\n");
        format!(
            "{}\n{}\n{}",
            if self.border.top {
                "─".repeat(width)
            } else {
                String::new()
            }
            .bold(),
            content,
            if self.border.bottom {
                "─".repeat(width)
            } else {
                String::new()
            }
            .bold()
        )
    }

    pub fn render_raw(&self) -> String {
        let csr: Vec<(Vec<String>, usize)> = self
            .columns
            .iter()
            .map(|c| (c.render_raw(), c.get_render_width()))
            .collect_vec();
        let mut rows: Vec<Vec<String>> = Vec::new();
        let mut width = 0_usize;
        for (s, size) in csr {
            let mut pointer = 0_usize;
            for ss in s {
                if let Some(r) = rows.get_mut(pointer) {
                    r.push(ss);
                } else {
                    rows.push(vec![ss])
                }
                pointer += 1;
            }
            width += size;
        }
        width += 2;
        let content = rows
            .into_iter()
            .map(|row| {
                format!(
                    "{}{}{}",
                    if self.border.left { "|" } else { "" },
                    row.join(""),
                    if self.border.left { "|" } else { "" }
                )
            })
            .join("\n");
        format!(
            "{}\n{}\n{}",
            if self.border.top {
                "─".repeat(width)
            } else {
                String::new()
            },
            content,
            if self.border.bottom {
                "─".repeat(width)
            } else {
                String::new()
            }
        )
    }
}

#[test]
fn test_table_render() {
    use crate::table::{Cell, CellPosition};
    use colored::Colorize;

    let mut cells: Vec<Vec<TableCell>> = vec![
        vec![
            TableCell::new(Cell::TextCell("Cell Row".into())).with_width(20),
            TableCell::new(Cell::TextCell("Left".into()))
                .with_position(CellPosition::Left)
                .with_width(10),
            TableCell::new(Cell::TextCell("Middle".into()))
                .with_position(CellPosition::Middle)
                .with_width(10),
            TableCell::new(Cell::TextCell("Right".into()))
                .with_position(CellPosition::Right)
                .with_width(10),
        ],
        (0..4).map(|_| TableCell::new(Cell::Splitter)).collect_vec(),
    ];
    cells.append(
        &mut (0..=3_u8)
            .into_iter()
            .map(|r| {
                vec![
                    TableCell::new(Cell::TextCell(format!("Cell Row: {}", r)))
                        .with_formatter(vec![Colorize::bold]),
                    TableCell::new(Cell::TextCell("Left".into())).with_position(CellPosition::Left),
                    TableCell::new(Cell::TextCell("Middle".into()))
                        .with_position(CellPosition::Middle),
                    TableCell::new(Cell::TextCell("Right".into()))
                        .with_position(CellPosition::Right),
                ]
            })
            .collect_vec(),
    );
    let table = Table::from_cells(cells);
    let render_res = table.render();
    println!("{}", render_res);
}

#[test]
fn test_table_default_builder() {
    use crate::table::CellPosition;
    let table_header: Vec<TableCell> = vec![
        TableCell::new(Cell::TextCell("Cell Row".into())).with_width(20),
        TableCell::new(Cell::TextCell("Left".into()))
            .with_position(CellPosition::Left)
            .with_width(10),
        TableCell::new(Cell::TextCell("Middle".into()))
            .with_position(CellPosition::Middle)
            .with_width(10),
        TableCell::new(Cell::TextCell("Right".into()))
            .with_position(CellPosition::Right)
            .with_width(10),
    ];

    let table_cells: Vec<Vec<TableCell>> = (0..=3_u8)
        .into_iter()
        .map(|r| {
            vec![
                TableCell::new(Cell::TextCell(format!("Cell Row: {}", r)))
                    .with_formatter(vec![Colorize::bold]),
                TableCell::new(Cell::TextCell("Left".into())).with_position(CellPosition::Left),
                TableCell::new(Cell::TextCell("Middle".into())).with_position(CellPosition::Middle),
                TableCell::new(Cell::TextCell("Right".into())).with_position(CellPosition::Right),
            ]
        })
        .collect_vec();
    let table = Table::from_data(table_header, table_cells);
    let render_res = table.render();
    println!("{}", render_res);
}

#[test]
fn test_table_default_builder_raw() {
    use crate::table::CellPosition;
    let table_header: Vec<TableCell> = vec![
        TableCell::new(Cell::TextCell("Cell Row".into())).with_width(20),
        TableCell::new(Cell::TextCell("Left".into()))
            .with_position(CellPosition::Left)
            .with_width(10),
        TableCell::new(Cell::TextCell("Middle".into()))
            .with_position(CellPosition::Middle)
            .with_width(10),
        TableCell::new(Cell::TextCell("Right".into()))
            .with_position(CellPosition::Right)
            .with_width(10),
    ];

    let table_cells: Vec<Vec<TableCell>> = (0..=3_u8)
        .into_iter()
        .map(|r| {
            vec![
                TableCell::new(Cell::TextCell(format!("Cell Row: {}", r)))
                    .with_formatter(vec![Colorize::bold]),
                TableCell::new(Cell::TextCell("Left".into())).with_position(CellPosition::Left),
                TableCell::new(Cell::TextCell("Middle".into())).with_position(CellPosition::Middle),
                TableCell::new(Cell::TextCell("Right".into())).with_position(CellPosition::Right),
            ]
        })
        .collect_vec();
    let table = Table::from_data(table_header, table_cells).with_border(Border::NONE);
    let render_res = table.render_raw();
    println!("{:?}", render_res);
    println!("{}", render_res);
}
