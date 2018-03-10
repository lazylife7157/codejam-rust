extern crate codejam;

use codejam::utils::prelude::*;

fn main() {
    let mut input = Scanner::from(std::io::stdin());
    let t = input.next_line_as_number().unwrap();
    let result = input.next_n_lines(t).into_iter()
        .map(solve)
        .enumerate()
        .map(format_single_line)
        .collect::<Vec<String>>()
        .join("\n");

    println!("{}", result);
}

fn solve(s: String) -> String {
    s
}
