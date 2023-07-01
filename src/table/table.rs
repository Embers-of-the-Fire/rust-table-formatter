use std::io;

use colored::Colorize;
use itertools::Itertools;

use crate::error::TableError;
use crate::table::{Align, Border, Cell, Content, Overflow, Renderer};

use super::FormatterFunc;

/// This is the main entry point of the lib, which represents the table to render.
///
/// For more information, please see the lib's documentation.
#[derive(Clone)]
pub struct Table {
    table: Vec<Vec<Cell>>,
    border: Border,
}

impl Table {
    /// Create a new table with a header and some rows.
    ///
    /// When `splitter` is set to true, this will automatically add a splitter between header and contents.
    ///
    /// > This is the recommended way to create a new table, so for details see the lib's documentation.
    pub fn create(header: Vec<Cell>, mut cell: Vec<Vec<Cell>>, splitter: bool) -> Table {
        let mut v = if splitter {
            let dat = header
                .iter()
                .map(|_| {
                    Cell::default()
                        .with_content(Content::Splitter)
                        .with_formatter(vec![FormatterFunc::Normal(Colorize::bold)])
                })
                .collect_vec();
            let mut v = vec![header
                .into_iter()
                .map(|c| c.with_formatter(vec![FormatterFunc::Normal(Colorize::bold)]))
                .collect_vec()];
            v.push(dat);
            v
        } else {
            vec![header]
        };
        v.append(&mut cell);
        Self {
            table: v,
            border: Border::NONE,
        }
    }

    /// Create a new table with some rows.
    pub fn new(table: Vec<Vec<Cell>>) -> Table {
        Self {
            table,
            border: Border::NONE,
        }
    }

    pub fn with_border(mut self, border: Border) -> Table {
        self.border = border;
        self
    }
    pub fn set_border(&mut self, border: Border) {
        self.border = border;
    }

    /// This function will overwrite the `overflow` property of every cells in the table.
    pub fn overwrite_overflow(&mut self, overflow: Overflow) {
        for row in self.table.iter_mut() {
            for cell in row.iter_mut() {
                cell.set_overflow(overflow)
            }
        }
    }

    /// This will render the table according to the render settings. See the lib's documentation for more information.
    ///
    /// See [Renderer]
    ///
    /// [Renderer]: ../enum.Renderer.html
    pub fn rendered_by(
        &self,
        setting: Renderer,
        writer: &mut impl io::Write,
    ) -> Result<(), TableError> {
        match setting {
            Renderer::Normal => self.render(writer),
            Renderer::Raw => self.render_raw(writer),
            Renderer::Markdown => self.render_markdown(writer),
        }
    }

    /// This will render a markdown-formatted table.
    ///
    /// See also [rendered_by].
    ///
    /// [rendered_by]: #method.rendered_by
    pub fn render_markdown(&self, writer: &mut impl io::Write) -> Result<(), TableError> {
        self.validate()?;
        let mut rows = self.table.iter();
        if let Some(h) = rows.next() {
            let setting_row = h
                .iter()
                .map(|c| match c.get_align() {
                    Align::Left => ":--",
                    Align::Center => ":-:",
                    Align::Right => "--:",
                })
                .join("┃");
            let hr = h
                .iter()
                .map(|c| match c.get_content() {
                    Content::None => "".to_string(),
                    Content::Splitter => "━━━".to_string(),
                    Content::Text(ref text) => text.clone(),
                })
                .join("┃");
            writeln!(writer, "┃{}┃", hr)?;
            writeln!(writer, "┃{}┃", setting_row)?;
            for row in rows {
                let hr = row
                    .iter()
                    .map(|c| match c.get_content() {
                        Content::None => "".to_string(),
                        Content::Splitter => "━━━".to_string(),
                        Content::Text(ref text) => text.clone(),
                    })
                    .join("┃");
                writeln!(writer, "┃{}┃", hr)?;
            }
        } else {
            writeln!(writer, "┃┃\n┃-┃")?;
        }
        Ok(())
    }

    /// This will render a raw table without any formatting.
    ///
    /// See also [rendered_by].
    ///
    /// [rendered_by]: #method.rendered_by
    pub fn render_raw(&self, writer: &mut impl io::Write) -> Result<(), TableError> {
        let w = self.validate()?;
        let widths = self.update_width(w)?;
        let table_width = widths.iter().map(|v| v + 2).sum::<usize>()
            + if self.border.left { 1 } else { 0 }
            + if self.border.right { 1 } else { 0 };
        if self.border.top {
            writeln!(writer, "{}", "━".repeat(table_width))?;
        }
        for (y, row) in self.table.iter().enumerate() {
            if self.border.left {
                write!(writer, "┃")?;
            }
            let mut pass: Vec<usize> = vec![];
            'cell: for (x, cell) in row.iter().enumerate() {
                if pass.binary_search(&x).is_ok() {
                    continue 'cell;
                }
                if let Some(cell_width) = widths.get(x) {
                    let render_width = if let Some(merge_width) = cell.get_merge() {
                        let cells = row.get((x + 1)..=(x + merge_width));
                        if let Some(merged) = cells {
                            let mut mv_width = vec![];
                            for (mx, cm) in merged.iter().enumerate() {
                                if !matches!(cm.get_content(), Content::None) {
                                    Err(TableError::MergeOverwrite { x: x + mx + 1, y })?
                                }
                                if let Some(mw) = widths.get(mx + x + 1) {
                                    mv_width.push(mw + 2);
                                } else {
                                    Err(TableError::Unexpected(
                                        "Render merge failed(index error)".to_string(),
                                    ))?
                                }
                                pass.push(x + mx + 1);
                            }
                            mv_width.into_iter().sum::<usize>() + cell_width
                        } else {
                            Err(TableError::MergeOutOfTable {
                                x,
                                y,
                                merge: merge_width,
                                max: w - x - 1,
                            })?
                        }
                    } else {
                        *cell_width
                    };
                    let rendered = cell.render_with_width_raw(render_width);
                    write!(writer, "{}", rendered)?;
                } else {
                    Err(TableError::Unexpected(
                        "Getting width failed(index error)".to_string(),
                    ))?
                }
            }
            if self.border.right {
                write!(writer, "┃")?;
            }
            writeln!(writer)?;
        }
        if self.border.bottom {
            writeln!(writer, "{}", "━".repeat(table_width))?;
        }
        Ok(())
    }

    /// This will render a table with formatting you defined.
    ///
    /// See also [rendered_by].
    ///
    /// [rendered_by]: #method.rendered_by
    pub fn render(&self, writer: &mut impl io::Write) -> Result<(), TableError> {
        let w = self.validate()?;
        let widths = self.update_width(w)?;
        let table_width = widths.iter().map(|v| v + 2).sum::<usize>()
            + if self.border.left { 1 } else { 0 }
            + if self.border.right { 1 } else { 0 };
        if self.border.top {
            writeln!(writer, "{}", "━".repeat(table_width).bold())?;
        }
        for (y, row) in self.table.iter().enumerate() {
            if self.border.left {
                write!(writer, "{}", "┃".bold())?;
            }
            let mut pass: Vec<usize> = vec![];
            'cell: for (x, cell) in row.iter().enumerate() {
                if pass.binary_search(&x).is_ok() {
                    continue 'cell;
                }
                if let Some(cell_width) = widths.get(x) {
                    let render_width = if let Some(merge_width) = cell.get_merge() {
                        let cells = row.get((x + 1)..=(x + merge_width));
                        if let Some(merged) = cells {
                            let mut mv_width = vec![];
                            for (mx, cm) in merged.iter().enumerate() {
                                if !matches!(cm.get_content(), Content::None) {
                                    Err(TableError::MergeOverwrite { x: x + mx + 1, y })?
                                }
                                if let Some(mw) = widths.get(mx + x + 1) {
                                    mv_width.push(mw + 2);
                                } else {
                                    Err(TableError::Unexpected(
                                        "Render merge failed(index error)".to_string(),
                                    ))?
                                }
                                pass.push(x + mx + 1);
                            }
                            mv_width.into_iter().sum::<usize>() + cell_width
                        } else {
                            Err(TableError::MergeOutOfTable {
                                x,
                                y,
                                merge: merge_width,
                                max: w - x - 1,
                            })?
                        }
                    } else {
                        *cell_width
                    };
                    let rendered = cell.render_with_width(render_width);
                    write!(writer, "{}", rendered)?;
                } else {
                    Err(TableError::Unexpected(
                        "Getting width failed(index error)".to_string(),
                    ))?
                }
            }
            if self.border.right {
                write!(writer, "{}", "┃".bold())?;
            }
            writeln!(writer)?;
        }
        if self.border.bottom {
            writeln!(writer, "{}", "━".repeat(table_width).bold())?;
        }
        Ok(())
    }

    fn update_width(&self, w: usize) -> Result<Vec<usize>, TableError> {
        let mut v = std::iter::repeat(0).take(w).collect_vec();
        for row in self.table.iter() {
            for (index, cell) in row.iter().enumerate() {
                if let Some(c) = v.get_mut(index) {
                    if let Some(iw) = cell.get_width() {
                        if iw > *c {
                            *c = iw;
                        }
                    }
                } else {
                    Err(TableError::Unexpected(
                        "Update width failed(index error)".to_string(),
                    ))?
                }
            }
        }
        Ok(v)
    }

    /// Check if the table is valid. The `usize` represents how many columns the table has.
    ///
    /// > This will be automatically checked when rendering, but you could also check it manually before it renders.
    pub fn validate(&self) -> Result<usize, TableError> {
        let mut t = self.table.iter();
        if let Some(v) = t.next() {
            let e = v.len();
            for (index, iv) in t.enumerate() {
                let l = iv.len();
                if l != e {
                    return Err(TableError::InvalidTableSize {
                        row: index + 1,
                        expected: e,
                        actual: l,
                    });
                }
            }
            Ok(e)
        } else {
            Ok(0)
        }
    }
}

#[test]
fn test_merge_cell() {
    let table = Table::new(vec![
        (0..5)
            .map(|_| {
                Cell::default()
                    .with_content(Content::Splitter)
                    .with_width(Some(5))
            })
            .collect_vec(),
        vec![
            Cell::default().with_content(Content::Splitter),
            Cell::default()
                .with_content(Content::new("123123123"))
                .with_merge(Some(2)),
            Cell::default().with_content(Content::None),
            Cell::default().with_content(Content::None),
            Cell::default().with_content(Content::Splitter),
        ],
    ]);

    let mut v = vec![];
    let r = table.render(&mut v);
    if let Err(e) = r {
        println!("{}", e);
    } else {
        println!("{}", String::from_utf8(v).unwrap());
    }
}
