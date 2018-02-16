use ::utils::Problem;
use ::utils::scan::Scanner;
use ::utils::geometry::prelude::*;
use std::f64::consts::PI;

pub struct C {
    f: f64,
    r: f64,
    t: f64,
    r2: f64,
    g: f64,

    interval: f64,
    safe_g: f64,
    inner_r: f64,
    unsafe_r2: f64
}

impl<'a> From<&'a mut Scanner> for C {
    fn from(scanner: &'a mut Scanner) -> C {
        let numbers = scanner.next_numbers();

        C {
            f: numbers[0],
            r: numbers[1],
            t: numbers[2],
            r2: numbers[3],
            g: numbers[4],

            interval: numbers[4] + numbers[3] * 2.0,
            safe_g: numbers[4] - numbers[0] * 2.0,
            inner_r: numbers[1] - (numbers[2] + numbers[0]),
            unsafe_r2: numbers[3] + numbers[0]
        }
    }
}

impl Problem for C {
    fn solve(&self) -> String {
        if self.safe_g > 0.0 {
            let probability = 1.0 - self.get_safe_area() / (PI * self.r.powi(2));
            format!("{:0.6}", probability)
        } else {
            // There is no chance to survive.
            format!("{:0.6}", 1.0)
        }
    }
}

impl C {
    fn get_x_or_y(&self, y_or_x: f64) -> f64 {
        (self.inner_r.powi(2) - y_or_x.powi(2)).sqrt()
    }

    fn get_arc_area(&self, arc_a: Point, arc_b: Point) -> f64 {
        let arc_c = center(arc_a, arc_b);
        let right_triangle = RightTriangle::from((O, arc_b, arc_c));
        let circular_sector = self.inner_r.powi(2) * right_triangle.theta();
        let triangle = right_triangle.area() * 2.0_f64;

        circular_sector - triangle
    }

    fn get_safe_area(&self) -> f64 {
        // Calculate safe area for 1/4 of the racquet.
        let mut safe_area = 0.0;
        let mut x = self.unsafe_r2;
        while x < self.inner_r {
            let mut y = self.unsafe_r2;
            while y < self.inner_r {
                safe_area += self.get_single_safe_area(x, y);
                y += self.interval;
            }
            x += self.interval;
        }
        safe_area * 4.0
    }

    fn get_single_safe_area(&self, x: f64, y: f64) -> f64 {
        let a = if x < y { (x, y) } else { (y, x) };
        let b = (a.0 + self.safe_g, a.1);
        let c = (b.0, a.1 + self.safe_g);
        let d = (a.0, a.1 + self.safe_g);

        let mut safe_area = 0.0_f64;
        if dist(O, a) < self.inner_r {
            if dist(O, c) < self.inner_r {
                // Safe area is inside the ring.
                //     d┌────┐c
                //      │    │
                //     a└────┘b
                //
                // O
                safe_area += self.safe_g.powi(2);

            } else if d.1 < self.get_x_or_y(a.0) {
                // c is outside the ring.
                let bc = (b.0, self.get_x_or_y(b.0));
                let cd = (self.get_x_or_y(d.1), d.1);
                let ad = (a.0, bc.1);

                safe_area += Rectangle::from((a, b, bc, ad)).area();
                safe_area += Trapezoid::from((bc, ad, cd, d)).area();
                safe_area += self.get_arc_area(bc, cd);

            } else if b.1 < self.get_x_or_y(b.0) {
                // c, d are outside the ring.
                let bc = (b.0, self.get_x_or_y(b.0));
                let ad = (a.0, self.get_x_or_y(a.0));

                safe_area += Trapezoid::from((ad, a, b, bc)).area();
                safe_area += self.get_arc_area(bc, ad);

            } else {
                // b, c, d are outside the ring.
                let ab = (self.get_x_or_y(a.1), a.1);
                let ad = (a.0, self.get_x_or_y(a.0));
                safe_area += RightTriangle::from((ad, ab, a)).area();
                safe_area += self.get_arc_area(ab, ad);
            }
        }
        safe_area
    }
}
