use super::super::prelude::*;

pub struct Trapezoid {
    a: Point,
    b: Point,
    c: Point,
    d: Point
}

impl From<(Point, Point, Point, Point)> for Trapezoid {
    fn from(pts: (Point, Point, Point, Point)) -> Trapezoid {
        Trapezoid { a: pts.0, b: pts.1, c: pts.2, d: pts.3 }
    }
}

impl Shape for Trapezoid {
    fn area(&self) -> f64 {
        let width = (dist(self.a, self.b) + dist(self.c, self.d)) / 2.0_f64;
        let height = Line::from((self.a, self.b)).dist_from(self.d);
        width * height
    }
}
