extern crate codejam;

use codejam::utils::prelude::*;

fn main() {
    let mut input = Scanner::from(Source::Stdin);
    let t = input.next_number();
    let result = input.next_n_lines(t)
        .map(solve)
        .enumerate()
        .map(format_single_line)
        .collect<Vec<String>>()
        .join("\n");

    println!("{}", result);
}

fn solve(s: String) -> String {
    s
}
