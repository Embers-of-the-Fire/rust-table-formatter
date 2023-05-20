use itertools::Itertools;

use crate::table::{Cell, CellPosition};

use super::{CellOverflow, TableCell, TableColumn};

#[derive(Debug, Clone)]
pub struct Table {
    columns: Vec<TableColumn>,
    header: bool,
    footer: bool,
}

impl Table {
    pub fn new(columns: Vec<TableColumn>) -> Table {
        Table {
            columns,
            header: false,
            footer: false,
        }
    }
    pub fn with_header(mut self) -> Self {
        self.header = true;
        self
    }
    pub fn with_footer(mut self) -> Self {
        self.footer = true;
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

    pub fn render(&self) -> String {
        let csr: Vec<Vec<String>> = self.columns.iter().map(|c| c.render()).collect_vec();
        let mut rows: Vec<Vec<String>> = Vec::new();
        for s in csr {
            let mut pointer = 0_usize;
            for ss in s {
                if let Some(r) = rows.get_mut(pointer) {
                    r.push(ss);
                } else {
                    rows.push(vec![ss])
                }
                pointer += 1;
            }
        }
        let width = if let Some(w) = rows.get(0) {
            w.iter().map(|s| s.chars().map(|_| 1_usize).sum::<usize>()).sum::<usize>() + 2
        } else {
            0
        };
        let content = rows.into_iter().map(|row| format!("|{}|", row.join(""))).join("\n");
        format!("{}\n{}\n{}", "─".repeat(width), content, "─".repeat(width))
    }
}

#[test]
fn test_table_render() {
    let mut cells: Vec<Vec<TableCell>> = vec![
        vec![
            TableCell::new(Cell::TextCell("Cell Row".into())).with_width(20),
            TableCell::new(Cell::TextCell("Left".into())).with_position(CellPosition::Left).with_width(10),
            TableCell::new(Cell::TextCell("Middle".into())).with_position(CellPosition::Middle).with_width(10),
            TableCell::new(Cell::TextCell("Right".into())).with_position(CellPosition::Right).with_width(10),
        ],
        (0..4).map(|_| TableCell::new(Cell::Splitter)).collect_vec(),
    ];
    cells.append(
        &mut (0..=3_u8)
            .into_iter()
            .map(|r| {
                vec![
                    TableCell::new(Cell::TextCell(format!("Cell Row: {}", r))),
                    TableCell::new(Cell::TextCell("Left".into())).with_position(CellPosition::Left),
                    TableCell::new(Cell::TextCell("Middle".into()))
                        .with_position(CellPosition::Middle),
                    TableCell::new(Cell::TextCell("Right".into()))
                        .with_position(CellPosition::Right),
                ]
            })
            .collect_vec(),
    );
    let table = Table::from_cells(cells).with_header().with_footer();
    let render_res = table.render();
    println!("{}", render_res);
}
