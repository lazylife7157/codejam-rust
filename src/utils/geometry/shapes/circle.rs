use super::super::prelude::*;

pub struct Circle {
    o: Point,
    r: f64
}

impl From<(Point, f64)> for Circle {
    fn from(circle: (Point, f64)) -> Circle {
        Circle {
            o: circle.0,
            r: circle.1
        }
    }
}

impl From<f64> for Circle {
    fn from(r: f64) -> Circle {
        Circle::from((O, r))
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        PI * self.r.powi(2)
    }
}
