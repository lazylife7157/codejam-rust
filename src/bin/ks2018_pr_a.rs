extern crate codejam;

use codejam::utils::prelude::*;

fn main() {
    let mut input = Scanner::from(std::io::stdin());
    let t: usize = input.next_line_as_number().unwrap();
    let result = (0..t)
        .map(|_| next_case(&mut input))
        .map(solve)
        .enumerate()
        .map(format_single_line)
        .collect::<Vec<String>>()
        .join("\n");

    println!("{}", result);
}

fn solve(case: Case) -> String {
    let result = case.cities.iter().map(|city| {
        (0..case.n)
            .filter(|i| &case.a[*i] <= city && city <= &case.b[*i])
            .count()
            .to_string()
    }).collect::<Vec<String>>().join(" ");

    result
}

struct Case {
    n: usize,
    a: Vec<u32>,
    b: Vec<u32>,
    cities: Vec<u32>
}

fn next_case(input: &mut Scanner<Stdin>) -> Case {
    let n = input.next_line_as_number().unwrap();

    let mut a = Vec::new();
    let mut b = Vec::new();
    for (i, x) in input.next_line_as_numbers().into_iter().enumerate() {
        match i % 2 {
            0 => a.push(x),
            _ => b.push(x)
        }
    }
    let p = input.next_line_as_number().unwrap();
    let cities = (0..p)
        .filter_map(|_| input.next_line_as_number())
        .collect();

    input.skip_line();
    Case {
        n: n,
        a: a,
        b: b,
        cities: cities
    }
}
