//! This library is designed to render text tables with possible formatting.
//!
//! There is only one entry point to this library, the [Table] struct.
//!
//! # How to use
//!
//! ## Basic Usage
//!
//! You can use the macros to quickly create a table.
//!
//! ```rust
//! use itertools::Itertools;
//! use table_formatter::{cell, table};
//! use table_formatter::table::{Align, Border};
//! let table_header = vec![
//!     cell!("Cell Row").with_width(Some(20)),
//!     cell!("Left", align = Align::Left).with_width(Some(10)),
//!     cell!("Center", align = Align::Center).with_width(Some(10)),
//!     cell!("Right", align = Align::Right).with_width(Some(10)),
//! ];
//! let table_cells = {
//!     let mut v = (0..=3_u8)
//!         .map(|_| {
//!             vec![
//!                 cell!("Cell Row"),
//!                 cell!("Left", align = Align::Left),
//!                 cell!("Center", align = Align::Center),
//!                 cell!("Right", align = Align::Right),
//!             ]
//!         })
//!         .collect_vec();
//!     v.push(cell!("Cross Cell!", align = Align::Center).with_span(3));
//!     v
//! };
//! let table = table! {
//!     table_header
//!     ---
//!     table_cells
//!     with Border::ALL
//! };
//! let mut buffer = vec![];
//! table.render(&mut buffer).unwrap();
//! println!("{}", String::from_utf8(buffer).unwrap());
//! ```
//!
//! ## Output Definition
//!
//! You can use [table::Renderer] to define the way it renders.
//!
//! For example, such code below could generate a markdown table.
//!
//! ```rust
//! # use itertools::Itertools;
//! # use table_formatter::table::{Align, Border, Renderer};
//! # use table_formatter::{cell, table};
//! let table_header = vec![
//!     cell!("Cell Row").with_width(Some(20)),
//!     cell!("Right", align = Align::Right).with_width(Some(10)),
//! ];
//! let table_cells = (0..=3_u8)
//!     .map(|_| vec![cell!("Cell Row"), cell!("Right", align = Align::Right)])
//!     .collect_vec();
//! let table = table! {
//!     table_header
//!     ---
//!     table_cells
//!     with Border::ALL
//! };
//! let mut buffer = vec![];
//! table.rendered_by(Renderer::Markdown, &mut buffer).unwrap();
//! println!("{}", String::from_utf8(buffer).unwrap());
//! ```
//! 
//! Output:
//! ```markdown
//! |Cell Row|Right|
//! |:--|--:|
//! |───|───|
//! |Cell Row|Right|
//! |Cell Row|Right|
//! |Cell Row|Right|
//! |Cell Row|Right|
//! ```
//!
//! [table::Renderer]: ../table/enum.Renderer.html
//! [Table]: ../table/struct.Table.html

pub mod builder;
pub mod error;
pub mod table;
