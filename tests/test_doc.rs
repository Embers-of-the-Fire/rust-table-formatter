#[test]
fn doc_test() {
    use itertools::Itertools;
    use table_formatter::{cell, table};
    use table_formatter::table::{Align, Border};
    let table_header = vec![
        cell!("Cell Row").with_width(Some(20)),
        cell!("Left", align = Align::Left).with_width(Some(10)),
        cell!("Center", align = Align::Center).with_width(Some(10)),
        cell!("Right", align = Align::Right).with_width(Some(10)),
    ];
    let table_cells = {
        let mut v = (0..=3_u8)
            .map(|_| {
                vec![
                    cell!("Cell Row"),
                    cell!("Left", align = Align::Left),
                    cell!("Center", align = Align::Center),
                    cell!("Right", align = Align::Right),
                ]
            })
            .collect_vec();
        v.push(cell!("Cross Cell!", align = Align::Center).with_span(3));
        v
    };
    let table = table! {
        table_header
        ---
        table_cells
        with Border::ALL
    };
    let mut buffer = vec![];
    table.render(&mut buffer).unwrap();
    println!("{}", String::from_utf8(buffer).unwrap());
}
