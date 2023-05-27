use colored::Colorize;
use itertools::Itertools;

use super::{Cell, CellOverflow, TableCell, TableColumn};

pub struct Table {
    columns: Vec<TableColumn>,
    header: bool,
    footer: bool,
}

impl Table {
    pub fn new(columns: Vec<TableColumn>) -> Table {
        Table {
            columns,
            header: true,
            footer: true,
        }
    }
    pub fn with_header(mut self, header: bool) -> Self {
        self.header = header;
        self
    }
    pub fn with_footer(mut self, footer: bool) -> Self {
        self.footer = footer;
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
            .map(|row| format!("{}{}{}", "|".bold(), row.join(""), "|".bold()))
            .join("\n");
        format!(
            "{}\n{}\n{}",
            if self.header {
                "─".repeat(width).bold()
            } else {
                String::new().bold()
            }
            .bold(),
            content,
            if self.footer {
                "─".repeat(width).bold()
            } else {
                String::new().bold()
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
            .map(|row| format!("{}{}{}", "|", row.join(""), "|"))
            .join("\n");
        format!(
            "{}\n{}\n{}",
            if self.header {
                "─".repeat(width)
            } else {
                String::new()
            }
            .bold(),
            content,
            if self.footer {
                "─".repeat(width)
            } else {
                String::new()
            }
            .bold()
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
    let table = Table::from_data(table_header, table_cells);
    let render_res = table.render_raw();
    println!("{}", render_res);
}