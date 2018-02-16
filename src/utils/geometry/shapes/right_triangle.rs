use super::super::prelude::*;

pub struct RightTriangle {
    a: Point,
    b: Point,
    c: Point
}

impl From<(Point, Point, Point)> for RightTriangle {
    fn from(pts: (Point, Point, Point)) -> RightTriangle {
        RightTriangle { a: pts.0, b: pts.1, c: pts.2 }
    }
}

impl Shape for RightTriangle {
    fn area(&self) -> f64 {
        dist(self.a, self.c) * dist(self.b, self.c) / 2.0_f64
    }
}

impl RightTriangle {
    fn opposite(&self) -> Line {
        Line::from((self.b, self.c))
    }

    fn adjacent(&self) -> Line {
        Line::from((self.a, self.c))
    }

    fn hypotenuse(&self) -> Line {
        Line::from((self.a, self.b))
    }

    pub fn theta(&self) -> f64 {
        (dist(self.b, self.c) / dist(self.a, self.c)).atan()
    }
}
