extern crate codejam;

use codejam::utils::prelude::*;
use codejam::utils::geometry::prelude::*;

fn main() {
    let mut input = Scanner::from(std::io::stdin());
    let result = (0..input.next_line_as_number().unwrap())
        .map(|_| next_case(&mut input))
        .map(solve)
        .enumerate()
        .map(format_single_line)
        .collect::<Vec<String>>()
        .join("\n");

    println!("{}", result);
}

fn solve(case: Case) -> String {
    let probability =
        if case.safe_g > 0.0 {
            1.0 - get_safe_area(&case) / (PI * case.r.powi(2))
        } else { 1.0 };
    format!("{:0.6}", probability)
}

fn get_safe_area(case: &Case) -> f64 {
    // Calculate safe area for 1/4 of the racquet.
    let mut safe_area = 0.0;
    let mut x = case.unsafe_r2;
    while x < case.inner_r {
        let mut y = case.unsafe_r2;
        while y < case.inner_r {
            safe_area += get_single_safe_area(case, x, y);
            y += case.interval;
        }
        x += case.interval;
    }
    safe_area * 4.0
}

fn get_single_safe_area(case: &Case, x: f64, y: f64) -> f64 {
    let get_x_or_y = |y_or_x| _get_x_or_y(case.inner_r, y_or_x);
    let get_arc_area = |arc_a, arc_b| _get_arc_area(case.inner_r, arc_a, arc_b);
    let a = if x < y { (x, y) } else { (y, x) };
    let b = (a.0 + case.safe_g, a.1);
    let c = (b.0, a.1 + case.safe_g);
    let d = (a.0, a.1 + case.safe_g);

    let mut safe_area = 0.0_f64;
    if dist(O, a) < case.inner_r {
        if dist(O, c) < case.inner_r {
            // Safe area is inside the ring.
            //     d┌────┐c
            //      │    │
            //     a└────┘b
            //
            // O
            safe_area += case.safe_g.powi(2);

        } else if d.1 < get_x_or_y(a.0) {
            // c is outside the ring.
            let bc = (b.0, get_x_or_y(b.0));
            let cd = (get_x_or_y(d.1), d.1);
            let ad = (a.0, bc.1);

            safe_area += Rectangle::from((a, b, bc, ad)).area();
            safe_area += Trapezoid::from((bc, ad, cd, d)).area();
            safe_area += get_arc_area(bc, cd);

        } else if b.1 < get_x_or_y(b.0) {
            // c, d are outside the ring.
            let bc = (b.0, get_x_or_y(b.0));
            let ad = (a.0, get_x_or_y(a.0));

            safe_area += Trapezoid::from((ad, a, b, bc)).area();
            safe_area += get_arc_area(bc, ad);

        } else {
            // b, c, d are outside the ring.
            let ab = (get_x_or_y(a.1), a.1);
            let ad = (a.0, get_x_or_y(a.0));
            safe_area += RightTriangle::from((ad, ab, a)).area();
            safe_area += get_arc_area(ab, ad);
        }
    }
    safe_area
}

fn _get_x_or_y(r: f64, y_or_x: f64) -> f64 {
    (r.powi(2) - y_or_x.powi(2)).sqrt()
}

fn _get_arc_area(r: f64, arc_a: Point, arc_b: Point) -> f64 {
    let arc_c = center(arc_a, arc_b);
    let right_triangle = RightTriangle::from((O, arc_b, arc_c));
    let circular_sector = r.powi(2) * right_triangle.theta();
    let triangle = right_triangle.area() * 2.0_f64;

    circular_sector - triangle
}



struct Case {
    r: f64,
    interval: f64,
    safe_g: f64,
    inner_r: f64,
    unsafe_r2: f64
}

fn next_case(input: &mut Scanner<Stdin>) -> Case {
    let numbers: Vec<f64> = input.next_line_as_numbers();
    Case {
        r: numbers[1],
        interval: numbers[4] + numbers[3] * 2.0,
        safe_g: numbers[4] - numbers[0] * 2.0,
        inner_r: numbers[1] - (numbers[2] + numbers[0]),
        unsafe_r2: numbers[3] + numbers[0]
    }
}
