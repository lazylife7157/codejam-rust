extern crate codejam;

use codejam::utils::prelude::*;

const PHRASE: &str = "welcome to code jam";
const LEN_PHRASE: usize = 19;

fn main() {
    let mut input = Scanner::from(std::io::stdin());
    let n = input.next_line_as_number().unwrap();
    let result: Vec<String> = input.next_n_lines(n).into_iter()
        .map(solve)
        .enumerate()
        .map(format_single_line)
        .collect();

    println!("{}", result.join("\n"));
}

fn solve(text: String) -> String {
    let mut x = vec![0; LEN_PHRASE];

    for c in text.chars() {
        for (i, d) in PHRASE.chars().enumerate() {
            if c == d {
                x[i] += if i == 0 { 1 } else { x[i-1] };
                x[i] %= 10000;
            }
        }
    }

    format!("{:04}", x[LEN_PHRASE - 1])
}
