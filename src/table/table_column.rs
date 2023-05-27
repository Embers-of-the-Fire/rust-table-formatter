use itertools::Itertools;

use super::{CellOverflow, TableCell};

pub struct TableColumn {
    cells: Vec<TableCell>,
    width: usize,
}

impl TableColumn {
    pub fn new(cells: Vec<TableCell>, width: usize) -> TableColumn {
        Self { cells, width }
    }
    pub fn with_overflow(mut self, overflow: CellOverflow) -> Self {
        self.cells.iter_mut().for_each(|c| c.set_overflow(overflow));
        self
    }
    pub fn set_overflow(&mut self, overflow: CellOverflow) {
        self.cells.iter_mut().for_each(|c| c.set_overflow(overflow));
    }
    pub fn with_width(mut self, width: usize) -> Self {
        self.width = width;
        self
    }
    pub fn set_width(&mut self, width: usize) {
        self.width = width;
    }
    pub fn get_width(&self) -> usize {
        self.width
    }
    pub fn get_render_width(&self) -> usize {
        self.width + 2
    }

    /// from row.
    pub fn from_cells(cells: Vec<Vec<TableCell>>) -> Vec<TableColumn> {
        let mut vec: Vec<Vec<TableCell>> = Vec::new();
        for cell_row in cells {
            let mut pointer = 0_usize;
            for cell in cell_row.into_iter() {
                if let Some(vcol) = vec.get_mut(pointer) {
                    vcol.push(cell)
                } else {
                    vec.push(vec![cell])
                }
                pointer += 1;
            }
        }
        vec.into_iter()
            .map(|cells| {
                let width = cells
                    .iter()
                    .fold(3_usize, |acc, cell| acc.max(cell.get_width().unwrap_or(1)));
                TableColumn::new(cells, width)
            })
            .collect_vec()
    }

    pub fn render(&self) -> Vec<String> {
        self.cells
            .iter()
            .map(|cell| cell.render(self.width))
            .collect_vec()
    }

    pub fn render_raw(&self) -> Vec<String> {
        self.cells
            .iter()
            .map(|cell| cell.render_raw(self.width))
            .collect_vec()
    }
}

#[test]
fn test_column_render() {
    use crate::table::{Cell, TableCell, TableColumn};
    use colored::Colorize;
    use itertools::Itertools;

    let cells: Vec<Vec<TableCell>> = (0..=3_u8)
        .into_iter()
        .map(|r| {
            (0..=5_u8)
                .into_iter()
                .map(|i| {
                    TableCell::new(Cell::TextCell(format!("Cell: {}@{}", r, i)))
                        .with_formatter(vec![Colorize::green])
                })
                .collect_vec()
        })
        .collect_vec();
    let column = TableColumn::from_cells(cells);
    let render_res = column.iter().map(|c| c.render()).collect_vec();
    println!("{:#?}", render_res);
}
