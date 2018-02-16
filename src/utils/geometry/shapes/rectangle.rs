use super::super::prelude::*;

pub struct Rectangle {
    a: Point,
    b: Point,
    c: Point,
    d: Point
}

impl From<(Point, Point, Point, Point)> for Rectangle {
    fn from(pts: (Point, Point, Point, Point)) -> Rectangle {
        Rectangle { a: pts.0, b: pts.1, c: pts.2, d: pts.3 }
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        dist(self.a, self.b) * dist(self.a, self.d)
    }
}
