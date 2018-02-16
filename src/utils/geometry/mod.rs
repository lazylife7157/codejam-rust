mod point;
mod line;
mod shapes;

pub mod prelude {
    pub use ::std::f64::*;
    pub use ::std::f64::consts::*;

    pub use super::*;
    pub use super::point::*;
    pub use super::line::*;

    pub use super::shapes::circle::*;
    pub use super::shapes::rectangle::*;
    pub use super::shapes::right_triangle::*;
    pub use super::shapes::trapezoid::*;
}

pub trait Shape {
    fn area(&self) -> f64;
}
