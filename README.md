# Tabel Formatter

This lib is used to format plain-text table.

## Example

Code:

```rust
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
            TableCell::new(Cell::TextCell(format!("Cell Row: {}", r))),
            TableCell::new(Cell::TextCell("Left".into())).with_position(CellPosition::Left),
            TableCell::new(Cell::TextCell("Middle".into())).with_position(CellPosition::Middle),
            TableCell::new(Cell::TextCell("Right".into())).with_position(CellPosition::Right),
        ]
    })
    .collect_vec();
let table = Table::from_data(table_header, table_cells);
let render_res = table.render();
println!("{}", render_res);
```

Output:

```
────────────────────────────────────────────────────────────
| Cell Row              Left          Middle         Right |
|──────────────────────────────────────────────────────────|
| Cell Row: 0           Left          Middle         Right |
| Cell Row: 1           Left          Middle         Right |
| Cell Row: 2           Left          Middle         Right |
| Cell Row: 3           Left          Middle         Right |
────────────────────────────────────────────────────────────
```

> Actually the head part of the table is bold, but it cannot be rendered in markdown.

## Future Plan

Write a macro for a better experience when building the table.

Support cross-cell text.

## License

[MIT][MIT-License] or [Apache-2.0][Apache-License]

[MIT-License]: LICENSE-MIT
[Apache-License]: LICENSE-APACHE
