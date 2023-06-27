use itertools::Itertools;
use table_formatter::{cell, table, table::*};

#[test]
fn test_table_formatter() {
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

#[test]
fn test_table_formatter_raw() {
    let table = Table::new(
        (0..=4)
            .map(|i1| {
                if i1 % 2 == 0 {
                    let mut v = vec![Cell::default().with_content(Content::Splitter)];
                    v.append(
                        &mut Cell::default()
                            .with_content(Content::new("123123123123123123123123123"))
                            .with_span(1),
                    );
                    v.push(Cell::default().with_content(Content::Splitter));
                    v
                } else {
                    (0..4)
                        .map(|i2| {
                            Cell::default().with_content(Content::new(format!("{}-{}", i1, i2)))
                        })
                        .collect_vec()
                }
            })
            .collect_vec(),
    )
    .with_border(Border::ALL);

    let mut r = Vec::new();
    let result = table.render_raw(&mut r);
    if let Err(e) = result {
        println!("{}", e)
    } else {
        println!("{}", String::from_utf8(r).unwrap())
    }
}

#[test]
fn test_table_formatter_markdown() {
    let table = Table::new(
        (0..=4)
            .map(|i1| {
                if i1 % 2 == 0 {
                    let mut v = vec![Cell::default().with_content(Content::Splitter)];
                    v.append(
                        &mut Cell::default()
                            .with_content(Content::new("123123123123123123123123123"))
                            .with_span(1),
                    );
                    v.push(Cell::default().with_content(Content::Splitter));
                    v
                } else {
                    (0..4)
                        .map(|i2| {
                            Cell::default().with_content(Content::new(format!("{}-{}", i1, i2)))
                        })
                        .collect_vec()
                }
            })
            .collect_vec(),
    )
    .with_border(Border::ALL);

    let mut r = Vec::new();
    let result = table.render_markdown(&mut r);
    if let Err(e) = result {
        println!("{}", e)
    } else {
        println!("{}", String::from_utf8(r).unwrap())
    }
}

#[test]
fn test_default() {
    use table_formatter::table::*;
    use table_formatter::{cell, table};
    let table_header = vec![
        vec![
            Cell::default().with_content(Content::new("123")),
            Cell::default().with_content(Content::new(444)),
        ],
        vec![
            Cell::default().with_content(Content::None),
            Cell::default().with_content(Content::new("777")),
        ],
    ];
    let table = table! {
        table_header
    };
    let mut buffer = vec![];
    table.render(&mut buffer).unwrap();
    println!("{}", String::from_utf8(buffer).unwrap());
}
