use std::io::Write;

use itertools::Itertools;

use crate::format_error::FormatError;

#[derive(Debug, Clone)]
pub struct Table {
    rows: Vec<TableRow>,
    column_width: Vec<usize>,
    table_width: usize,
}

impl Table {
    pub fn new(title: &[TableCell], content: &[TableRow]) -> Result<Table, FormatError> {
        let mut v = vec![TableRow::Row(title.to_vec()), TableRow::Splitter];
        v.append(&mut content.to_vec());
        let row_length = v
            .iter()
            .find_map(|v| match v {
                TableRow::Row(v) => Some(v.len()),
                _ => None,
            })
            .unwrap_or(0);
        let (err, column_width) = v.iter().fold((false, vec![0_usize; row_length]), |(mut err, mut acc), r| {
            if let TableRow::Row(rvec) = r {
                rvec.iter().enumerate().for_each(|(idx, v)| {
                    if let Some(c) = acc.get_mut(idx) {
                        let len = v.content().len();
                        if len > *c {
                            *c = len;
                        }
                    } else {
                        err = true;
                    }
                });
            }
            (err, acc)
        });
        if err {
            Err(FormatError::InconsistentRowWidth)?;
        }
        let table_width = column_width.iter().map(|v| v + 2).sum::<usize>() + 2;
        let table = Table {
            rows: v,
            table_width,
            column_width,
        };
        if table.validate() {
            Ok(table)
        } else {
            Err(FormatError::InconsistentRowWidth)
        }
    }

    pub fn with_table_width(mut self, table_width: usize) -> Self {
        self.table_width = table_width;
        self
    }

    pub fn validate(&self) -> bool {
        if let Some(r1) = self.rows.iter().find_map(|v| match v {
            TableRow::Row(v) => Some(v.len()),
            _ => None,
        }) {
            if matches!(
                self.rows.iter().find_map(|v| match v {
                    TableRow::Row(v) =>
                        if v.len() != r1 {
                            Some(())
                        } else {
                            None
                        },
                    _ => None,
                }),
                None
            ) {
                true
            } else {
                false
            }
        } else {
            true
        }
    }

    pub fn format<W: Write>(&self, writer: &mut W) -> Result<(), FormatError> {
        if !self.validate() {
            Err(FormatError::InconsistentRowWidth)?
        }
        writeln!(writer, "{}", "─".repeat(self.table_width))
            .map_err(|e| FormatError::WriteError(e.to_string()))?;
        for row in self.rows.iter() {
            match row {
                TableRow::Row(row) => {
                    if row.is_empty() {
                        continue;
                    }
                    let line_vec = row
                        .iter()
                        .enumerate()
                        .map(|(index, cell)| {
                            let cell_width = self.column_width[index];
                            match cell.align() {
                                TableAlign::Left => format!(
                                    "{}{}",
                                    cell.content(),
                                    " ".repeat(cell_width - cell.content().len())
                                ),
                                TableAlign::Right => format!(
                                    "{}{}",
                                    " ".repeat(cell_width - cell.content().len()),
                                    cell.content()
                                ),
                            }
                        })
                        .collect_vec();
                    let line_width = line_vec.iter().map(|v| v.len() + 2).sum::<usize>() - 4;
                    let line = if line_width < self.table_width - 4 {
                        let dif = self.table_width - line_width - 4;
                        let res_str = &line_vec[1..].join("  ");
                        let res = line_vec[0].clone();
                        res + " ".repeat(dif).as_str() + res_str.as_str()
                    } else {
                        line_vec.join("  ")
                    };
                    writeln!(writer, "| {} |", line)
                        .map_err(|e| FormatError::WriteError(e.to_string()))?;
                }
                TableRow::PlainText(text) => {
                    let text_len = text.len();
                    if text_len + 4 <= self.table_width {
                        writeln!(
                            writer,
                            "| {}{} |",
                            text,
                            " ".repeat(self.table_width - 4 - text_len)
                        )
                        .map_err(|e| FormatError::WriteError(e.to_string()))?;
                    } else {
                        for chunk in &text.chars().chunks(self.table_width - 4) {
                            let chunkl = chunk.collect::<String>();
                            writeln!(
                                writer,
                                "| {}{} |",
                                chunkl,
                                " ".repeat(self.table_width - 4 - chunkl.len())
                            )
                            .map_err(|e| FormatError::WriteError(e.to_string()))?
                        }
                    }
                }
                TableRow::Splitter => {
                    writeln!(writer, "{}", "─".repeat(self.table_width))
                        .map_err(|e| FormatError::WriteError(e.to_string()))?;
                }
                TableRow::BlankLine => {
                    writeln!(writer, "|{}|", " ".repeat(self.table_width - 2))
                        .map_err(|e| FormatError::WriteError(e.to_string()))?;
                }
            }
        }
        writeln!(writer, "{}", "─".repeat(self.table_width))
            .map_err(|e| FormatError::WriteError(e.to_string()))?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum TableRow {
    Row(Vec<TableCell>),
    PlainText(String),
    Splitter,
    BlankLine,
}

#[derive(Debug, Clone)]
pub struct TableCell {
    content: String,
    align: TableAlign,
}

impl TableCell {
    pub fn new<T: Into<String>>(content: T) -> TableCell {
        TableCell {
            content: content.into(),
            align: TableAlign::Left,
        }
    }

    pub fn with_align(mut self, align: TableAlign) -> TableCell {
        self.align = align;
        self
    }

    pub fn content(&self) -> &String {
        &self.content
    }

    pub fn align(&self) -> TableAlign {
        self.align
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TableAlign {
    Left,
    Right,
}
