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
