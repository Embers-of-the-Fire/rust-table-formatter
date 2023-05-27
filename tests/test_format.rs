use colored::Colorize;
use itertools::Itertools;
use table_formatter::table::Border;
use table_formatter::table::Cell;
use table_formatter::table::CellPosition;
use table_formatter::table::Table;
use table_formatter::table::TableCell;

#[test]
fn test_table_default_builder_raw() {
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
