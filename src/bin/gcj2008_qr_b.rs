extern crate codejam;

use codejam::utils::prelude::*;

fn main() {
    let mut input = Scanner::from(Source::Stdin);
    let n = input.next_number();
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
    let start_at_a = case.na - get_count_of_unnecessary_departures(case.t, &case.dep_a_to_b.as_slice(), &case.arr_b_to_a.as_slice());
    let start_at_b = case.nb - get_count_of_unnecessary_departures(case.t, &case.dep_b_to_a.as_slice(), &case.arr_a_to_b.as_slice());
    format!("{} {}", start_at_a, start_at_b)
}

fn get_count_of_unnecessary_departures(t: u32, dep: &[u32], arr: &[u32]) -> usize {
    match dep.len() {
        0 => 0,
        _ => {
            if arr.len() > 0 && dep[0] >= arr[0] + t {
                1 + get_count_of_unnecessary_departures(t, &dep[1..], &arr[1..])
            } else {
                get_count_of_unnecessary_departures(t, &dep[1..], arr)
            }
        }
    }
}



pub struct Case {
    t: u32,
    na: usize,
    nb: usize,
    dep_a_to_b: Vec<u32>,
    arr_a_to_b: Vec<u32>,
    dep_b_to_a: Vec<u32>,
    arr_b_to_a: Vec<u32>
}

fn next_case(input: &mut Scanner) -> Case {
    let t = input.next_number();
    let ns = input.next_numbers();
    let na = ns[0];
    let nb = ns[1];

    let mut get_timetables = |n| {
        let mut dep = Vec::new();
        let mut arr = Vec::new();
        for line in input.next_n_lines(n) {
            let times: Vec<String> = line.split_whitespace().map(String::from).collect();
            dep.push(time_to_min(&times[0]));
            arr.push(time_to_min(&times[1]));
        }
        dep.sort();
        arr.sort();
        (dep, arr)
    };

    let (dep_a_to_b, arr_a_to_b) = get_timetables(na);
    let (dep_b_to_a, arr_b_to_a) = get_timetables(nb);

    Case {
        t: t,
        na: na,
        nb: nb,
        dep_a_to_b: dep_a_to_b,
        arr_a_to_b: arr_a_to_b,
        dep_b_to_a: dep_b_to_a,
        arr_b_to_a: arr_b_to_a
    }
}

fn time_to_min(s: &String) -> u32 {
    let time: Vec<u32> = s.split(":").map(|t| t.parse().unwrap()).collect();
    time[0] * 60 + time[1]
}
