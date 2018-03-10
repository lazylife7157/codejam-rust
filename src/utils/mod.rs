pub mod geometry;
pub mod format;
pub mod scan;

pub mod prelude {
    pub use super::format::{format_single_line, format_multi_line};
    pub use super::scan::{Scan, Scanner};
    pub use std::io::Stdin;
}