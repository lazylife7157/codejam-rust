pub mod geometry;
pub mod scan;

pub trait Problem {
    fn solve(&self) -> String;
}
