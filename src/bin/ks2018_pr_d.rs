extern crate codejam;

use codejam::utils::prelude::*;

fn main() {
    let mut input = Scanner::from(std::io::stdin());
    let t: usize = input.next_line_as_number().unwrap();
    let result = (0..t)
        .map(|_| next_case(&mut input))
        .map(solve)
        .enumerate()
        .map(format_multi_line)
        .collect::<Vec<String>>()
        .join("\n");

    println!("{}", result);
}

fn solve(case: Case) -> String {
    case.sub_cases.iter().map(|&(left, right)| {
        let mut result = 0_u64;
        let mut element_idx = 1_u64;
        let mut value_idx;
        for (value, value_count) in case.new_array.iter().enumerate() {
            value_idx = 0_u64;
            if element_idx < left {
                for _ in value_idx..*value_count {
                    if element_idx < left {
                        value_idx += 1;
                        element_idx += 1;
                    } else {
                        break;
                    }
                }
            }

            if left <= element_idx && element_idx <= right {
                for _ in value_idx..*value_count {
                    if element_idx <= right {
                        result += value as u64;
                        element_idx += 1;
                    } else {
                        break;
                    }
                }
            }

            if element_idx > right {
                break;
            }
        }
        result.to_string()
    }).collect::<Vec<String>>().join("\n")
}

struct Case {
    new_array: Vec<u64>,
    sub_cases: Vec<(u64, u64)>
}

fn next_case(input: &mut Scanner<Stdin>) -> Case {
    let nq = input.next_line_as_numbers();
    let initial_array: Vec<usize> = input.next_line_as_numbers();
    let sub_cases = (0..nq[1])
        .map(|_| input.next_line_as_numbers())
        .map(|lr| (lr[0], lr[1]))
        .collect();

    let mut new_array = vec![0; 20_000_001];
    for j in 0..nq[0] {
        let mut x = 0;
        for k in 0..nq[0]-j {
            x += initial_array[j+k];
            new_array[x] += 1;
        }
    }

    Case {
        new_array: new_array,
        sub_cases: sub_cases
    }
}
