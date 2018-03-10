extern crate codejam;

use codejam::utils::prelude::*;

fn main() {
    let mut input = Scanner::from(std::io::stdin());
    let ldn: Vec<usize> = input.next_line_as_numbers();
    let words = input.next_n_lines(ldn[1]);
    let result = input.next_n_lines(ldn[2]).into_iter()
        .map(|pattern| solve(&words, pattern))
        .enumerate()
        .map(format_single_line)
        .collect::<Vec<String>>()
        .join("\n");

    println!("{}", result);
}

fn solve(words: &Vec<String>, pattern: String) -> String {
    let mut patterns = Vec::new();
    let mut sub_patterns = Vec::new();
    let mut is_sub_pattern = false;

    for ch in pattern.chars() {
        match ch {
            '(' => is_sub_pattern = true,
            ')' => {
                is_sub_pattern = false;
                sub_patterns.sort();
                patterns.push(sub_patterns.clone());
                sub_patterns.clear();
            },
            _  if is_sub_pattern => sub_patterns.push(ch),
            _  => patterns.push(vec![ch]),
        }
    }

    let is_matched = |word: &String|
        word.chars().enumerate()
            .find(|&(i, ch)| patterns[i].binary_search(&ch).is_err())
            .is_none();

    words.clone().into_iter()
        .filter(is_matched)
        .count()
        .to_string()
}
