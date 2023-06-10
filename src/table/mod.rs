//! Core library and the main entry point.

mod cell;
mod content;
mod settings;
#[allow(clippy::module_inception)]
mod table;

use std::rc::Rc;

pub use cell::*;
use colored::ColoredString;
pub use content::*;
pub use settings::*;
pub use table::*;

/// Wrapper for formatting-functions.
/// 
/// All functions could be converted to [Normal::Boxed], if you don't care about performance, try [fmt!] macro.
/// 
/// [fmt!]: ../macro.fmt.html
/// [Normal::Boxed]: #variant.Boxed
#[derive(Clone)]
pub enum FormatterFunc {
    Normal(fn(ColoredString) -> ColoredString),
    Boxed(Rc<Box<dyn Fn(ColoredString) -> ColoredString>>),
}

impl FormatterFunc {
    fn run(&self, s: ColoredString) -> ColoredString {
        match self {
            FormatterFunc::Normal(f) => f(s),
            FormatterFunc::Boxed(f) => f(s),
        }
    }
}
