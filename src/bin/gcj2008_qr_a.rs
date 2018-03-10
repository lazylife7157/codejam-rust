extern crate codejam;

use std::collections::HashSet;
use codejam::utils::prelude::*;

fn main() {
    let mut input = Scanner::from(std::io::stdin());
    let n = input.next_line_as_number().unwrap();
    let result = (0..n)
        .map(|_| next_case(&mut input))
        .map(solve)
        .enumerate()
        .map(format_single_line)
        .collect::<Vec<String>>()
        .join("\n");

    println!("{}", result);
}

fn solve(case: Case) -> String {
    let queries = case.queries.as_slice();
    let mut searched_engines = HashSet::new();
    get_min_switch(case.s, queries, &mut searched_engines).to_string()
}

fn get_min_switch(s: usize, queries: &[String], searched_engines: &mut HashSet<String>) -> u32 {
    match queries.len() {
        0 => 0,
        _ => {
            searched_engines.insert(queries[0].clone());
            if searched_engines.len() == s {
                searched_engines.clear();
                searched_engines.insert(queries[0].clone());
                1 + get_min_switch(s, &queries[1 ..], searched_engines)
            } else {
                get_min_switch(s, &queries[1 ..], searched_engines)
            }
        }
    }
}



struct Case {
    s: usize,
    queries: Vec<String>
}

fn next_case(input: &mut Scanner<Stdin>) -> Case {
    let s = input.next_line_as_number().unwrap();
    input.skip_n_lines(s);
    let q = input.next_line_as_number().unwrap();
    let queries = input.next_n_lines(q);

    Case {
        s: s,
        queries: queries
    }
}