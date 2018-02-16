use super::prelude::*;

pub struct Line {
    a: Point,
    b: Point
}

impl From<(Point, Point)> for Line {
    fn from(line: (Point, Point)) -> Line {
        Line {
            a: line.0,
            b: line.1
        }
    }
}

impl Line {
    pub fn len(&self) -> f64 {
        dist(self.a, self.b)
    }

    pub fn dist_from(&self, p: Point) -> f64 {
        let t1 = (self.b.1 - self.a.1) * p.0;
        let t2 = (self.b.0 - self.a.0) * p.1;
        let t3 = self.b.0 * self.a.1;
        let t4 = self.b.1 * self.a.0;
        let t5 = (self.b.1 - self.a.1).powi(2);
        let t6 = (self.b.0 - self.a.0).powi(2);
        (t1 - t2 + t3 - t4).abs() / (t5 + t6).sqrt()
    }
}
