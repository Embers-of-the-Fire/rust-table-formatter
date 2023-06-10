//! Macro definitions.

/// This macro is used to generate a table.
/// 
/// When you input two identifiers, this will call [Table::create]. If you add "-"s between identifiers, it will generate a splitter.
/// You can add `with Border` to add a [Border] definition.
/// 
/// If you just input a single expression, this will call [Table::new] directly.
/// You can also add `border = Border` to give a [Border] definition.
/// 
/// [Border]: ../table/struct.Border.html
/// [Table::create]: ../table/struct.Table.html#method.create
/// [Table::new]: ../table/struct.Table.html#method.new
#[macro_export]
macro_rules! table {
    {$h: ident $(-)+ $d: ident $(with $b: expr)?} => {
        $crate::table::Table::create($h, $d, true)$(.with_border($b))?
    };
    {$h: ident $d: ident $(with $b: expr)?} => {
        $crate::table::Table::create($h, $d, false)$(.with_border($b))?
    };
    {$d: expr $(, border = $b: expr)?} => {
        $crate::table::Table::new($d)$(.with_border($b))?
    };
}

/// This macro is used to generate a table cell.
/// 
/// If you input nothing, it will generate [Content::None].
/// 
/// If you input "-"s, it will generate [Content::Splitter].
/// 
/// If you input an expression, it will generate a new [Content::Text] by calling [Content::new]. You can also add alignment, padding and width definitions.
/// 
/// ```rust
/// # use table_formatter::cell;
/// # use table_formatter::table::{Align, Padding};
/// cell!("hello world", align=Align::Left, padding=Padding::NONE);
/// ```
/// 
/// [Content::None]: ../table/enum.Content.html#variant.None
/// [Content::Splitter]: ../table/enum.Content.html#variant.Splitter
/// [Content::Text]: ../table/enum.Content.html#variant.Text
/// [Content::new]: ../table/enum.Content.html#method.new
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

/// This macro is used to wrap those formatters.
/// 
/// Since the formatters use both closures and functions, and that two types are not capable with each other, the lib uses a enum(FormatterFunc) to combine them together.
/// 
/// This macro will automatically convert any function pointers and closures to a `Rc<Box<dyn Fn(...) -> ...>>`, generating [FormatterFunc::Boxed].
/// 
/// You can just use it as the way you use the `vec!` macro.
/// 
/// ```rust
/// # use table_formatter::fmt;
/// # use colored::Colorize;
/// let (r, g, b) = (100, 20, 70);
/// fmt!(Colorize::blue, move |s| s.truecolor(r, g, b));
/// ```
/// 
/// [FormatterFunc::Boxed]: ../table/enum.FormatterFunc.html#variant.Boxed
/// [FormatterFunc]: ../table/enum.FormatterFunc.html
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
