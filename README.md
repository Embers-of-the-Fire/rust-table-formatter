# Tabel Formatter

This lib is used to format plain-text table.

## Example

Code:

```rust
let data = [
    TableRow::Row(vec![
        TableCell::new("A"),
        TableCell::new("1").with_align(TableAlign::Right),
    ]),
    TableRow::Row(vec![
        TableCell::new("B"),
        TableCell::new("2").with_align(TableAlign::Right),
    ]),
    TableRow::Splitter,
    TableRow::PlainText("letter-number mapping".into()),
];
let header = [
    TableCell::new("Letter"),
    TableCell::new("Number").with_align(TableAlign::Right),
];
let table = Table::new(&header, &data).unwrap().with_align(30);
let mut output = Vec::new();
table.format(&mut output).unwrap();
println!("{}", String::from_utf8(output).unwrap());
```

Output:
```
──────────────────────────────
| Letter              Number |
──────────────────────────────
| A                        1 |
| B                        2 |
──────────────────────────────
| letter-number mapping      |
──────────────────────────────
```

## Future Plan

Write a macro for a better experience when building the table.

## License

[MIT][MIT-License] or [Apache-2.0][Apache-License]

[MIT-License]: LICENSE-MIT
[Apache-License]: LICENSE-APACHE
