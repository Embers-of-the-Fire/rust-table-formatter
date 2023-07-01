# Table Formatter

<p id="badge" align="center">
	<a href="https://github.com/Embers-of-the-Fire/rust-table-formatter"><img src="https://img.shields.io/badge/Repo-rust--table--formatter-FDC75B" alt="Repo" /></a>
    &nbsp;
    <a href="https://crates.io/crates/table_formatter"><img src="https://img.shields.io/crates/v/table_formatter
" alt="Crates-io Version" /></a>
    &nbsp;
    <img src="https://img.shields.io/github/languages/top/Embers-of-the-Fire/rust-table-formatter
" alt="Language Count" />
    &nbsp;
    <img src="https://img.shields.io/badge/License-Apache2.0%20%2F%20MIT-89B701
" alt="License" />
    <br />
    <a href="https://github.com/Embers-of-the-Fire/rust-table-formatter/actions"><img src="https://img.shields.io/github/actions/workflow/status/Embers-of-the-Fire/rust-table-formatter/mean_bean_ci.yml
" alt="Build Status" /></a>
    &nbsp;
    <a href="https://github.com/Embers-of-the-Fire/rust-table-formatter"><img src="https://img.shields.io/github/commit-activity/m/Embers-of-the-Fire/rust-table-formatter
" alt="Commit Activity" /></a>
    &nbsp;
    <a href="https://crates.io/crates/table_formatter"><img src="https://img.shields.io/crates/dr/table_formatter?logo=rust&color=FDC75B
" alt="Recent Downloads" /></a>
</p>

This lib is used to format plain-text table.

## Example

Code:

```rust
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
```

Output:

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
┃ Cell Row              Left          Center         Right ┃
┃━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┃
┃ Cell Row              Left          Center         Right ┃
┃ Cell Row              Left          Center         Right ┃
┃ Cell Row              Left          Center         Right ┃
┃ Cell Row              Left          Center         Right ┃
┃                       Cross Cell!                        ┃
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

> Actually the border of the table is bold, but it cannot be rendered in markdown.

## Future Plan

Waiting for report :)

## Change Log

### V0.6.0

-   Changed the api of formatter, and add a macro wrapper for them.
-   Added documentations.

> If you are using the formatter, you just need to change your `vec!`s into `fmt!`s.

### V0.5.1

This version is yanked.

### V0.5.0

#### New features

-   Add render target: Markdown.
-   Add cross-cell support.
-   Add macro support.

#### Warning

This version is **_Completely Incompatible_** with previous versions.

## License

[MIT][MIT-License] or [Apache-2.0][Apache-License]

[MIT-License]: LICENSE-MIT
[Apache-License]: LICENSE-APACHE
