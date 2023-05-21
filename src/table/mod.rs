mod overflow;
mod table;
mod table_cell;
mod table_column;

use colored::ColoredString;
pub use overflow::*;
pub use table::*;
pub use table_cell::*;
pub use table_column::*;

pub type FORMATTER = fn(ColoredString) -> ColoredString;
