extern crate codejam;

use std::collections::HashMap;
use codejam::utils::prelude::*;

fn main() {
    let mut input = Scanner::from(std::io::stdin());
    let t: usize = input.next_line_as_number().unwrap();
    let result = (0..t)
        .map(|_| {
            let n: usize = input.next_line_as_number().unwrap();
            let mut s2d = HashMap::new();
            let mut d2s = HashMap::new();
            for _ in 0..n {
                let src = input.next_line().unwrap();
                let dst = input.next_line().unwrap();
                s2d.insert(src.clone(), dst.clone());
                d2s.insert(dst.clone(), src.clone());
            }
            (s2d, d2s)
        })
        .map(solve)
        .enumerate()
        .map(format_single_line)
        .collect::<Vec<String>>()
        .join("\n");

    println!("{}", result);
}

fn solve((s2d, d2s): (HashMap<String, String>, HashMap<String, String>)) -> String {
    let an_airport = d2s.keys().next().unwrap().to_string();
    let mut sorted_tickets = Vec::new();
    sorted_tickets.append(&mut get_sorted_tickets(&d2s, &an_airport, true));
    sorted_tickets.append(&mut get_sorted_tickets(&s2d, &an_airport, false).into_iter().rev().collect());
    sorted_tickets.join(" ")
}

fn get_sorted_tickets(s2d: &HashMap<String, String>, airport: &String, reverse: bool) -> Vec<String> {
    if let Some(next_airport) = s2d.get(airport) {
        let mut tickets = get_sorted_tickets(s2d, next_airport, reverse);
        if reverse {
            tickets.push(format!("{}-{}", next_airport, airport))
        } else {
            tickets.push(format!("{}-{}", airport, next_airport))
        }
        tickets
    } else {
        Vec::new()
    }
}
