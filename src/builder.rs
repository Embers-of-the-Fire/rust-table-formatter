#[macro_export]
macro_rules! table {
    {$h: ident $(-)+ $d: ident $(with $b: expr)?} => {
        $crate::table::Table::create($h, $d, true)$(.with_border($b))?
    };
    {$h: ident $d: ident $(with $b: expr)?} => {
        $crate::table::Table::create($h, $d, false)$(.with_border($b))?
    };
    {$d: ident $(with $b: expr)?} => {
        $crate::table::Table::new($d)$(.with_border($b))?
    };
    ($d: expr) => {
        $crate::table::Table::new($d)
    };
}

#[macro_export]
macro_rules! cell {
    () => {
        $crate::table::Cell::default().with_content($crate::table::Content::None)
    };
    ($(-)+) => {
        $crate::table::Cell::default().with_content($crate::table::Content::Splitter)
    };
    ($d: expr $(, align=$align: expr)? $(, padding=$padding: expr)? $(, width=$width: expr)?) => {
        $crate::table::Cell::default().with_content($crate::table::Content::new($d))$(.with_align($align))?$(.with_padding($padding))?$(.with_width($width))?
    };
}

#[macro_export]
macro_rules! fmt {
    () => {
        vec![]
    };
    ($func: expr) => {
        vec![$crate::table::FormatterFunc::Boxed(std::rc::Rc::new(Box::new($func)))]
    };
    ($($func: expr),+ $(,)?) => {
        vec![$($crate::table::FormatterFunc::Boxed(std::rc::Rc::new(Box::new($func))), )*]
    }
}

#[test]
fn test_table_macro() {
    use colored::{Colorize, ColoredString};

    use crate::table::Align;
    use crate::table::Border;

    let (r, g, b) = (100, 0, 0);
    let dat1 = vec![cell!("left"), cell!("right", align = Align::Right)];
    let dat2 = vec![
        vec![cell!(-), cell!()],
        vec![
            cell!("1"),
            cell!("2", align = Align::Right).with_formatter(fmt!(move |s: ColoredString| s.truecolor(r, g, b), Colorize::blue)),
        ],
    ];
    let table = table! {dat1 - dat2 with Border::ALL};

    let mut renderer = vec![];
    table.render(&mut renderer).unwrap();
    println!("{}", String::from_utf8(renderer).unwrap());
}
