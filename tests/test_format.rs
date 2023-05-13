use table_formatter::table::{Table, TableAlign, TableCell, TableRow};

#[test]
fn test_format_table() {
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
    let table = Table::new(&header, &data).unwrap().with_table_width(30);
    let mut output = Vec::new();
    table.format(&mut output).unwrap();
    println!("{}", String::from_utf8(output).unwrap());
}
