pub type Point = (f64, f64);

pub const O: Point = (0.0_f64, 0.0_f64);

pub fn dist(a: Point, b: Point) -> f64 {
    ((a.0 - b.0).powi(2) + (a.1 - b.1).powi(2)).sqrt()
}

pub fn center(a: Point, b: Point) -> Point {
    (a.0 + (b.0 - a.0) / 2.0f64, a.1 + (b.1 - a.1) / 2.0f64)
}
