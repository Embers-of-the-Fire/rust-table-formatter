use itertools::Itertools;
use table_formatter::table::*;

#[test]
fn test_table_formatter() {
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
    let result = table.render(&mut r);
    if let Err(e) = result {
        println!("{}", e)
    } else {
        println!("{}", String::from_utf8(r).unwrap())
    }
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