mod overflow;
#[allow(clippy::module_inception)]
mod table;
mod table_cell;
mod table_column;
mod padding;

use colored::ColoredString;
pub use overflow::*;
pub use table::*;
pub use table_cell::*;
pub use table_column::*;
pub use padding::*;

pub type FORMATTER = fn(ColoredString) -> ColoredString;
