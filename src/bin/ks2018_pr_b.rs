extern crate codejam;

use codejam::utils::prelude::*;

fn main() {
    let mut input = Scanner::from(std::io::stdin());
    let t: usize = input.next_line_as_number().unwrap();
    let result = (0..t)
        .filter_map(|_| input.next_line_as_number())
        .map(solve)
        .enumerate()
        .map(format_single_line)
        .collect::<Vec<String>>()
        .join("\n");

    println!("{}", result);
}

fn solve(k: u64) -> String {
    match is_kth_character_zero(k) {
        true => 0.to_string(),
        false => 1.to_string()
    }
}

fn is_kth_character_zero(k: u64) -> bool {
    match k {
        1_u64 => true,
        _ => {
            let x = 2_u64.pow((k as f64).log2().floor() as u32);
            if k == x {
                true
            } else {
                !is_kth_character_zero(x - (k - x))
            }
        }
    }
}
