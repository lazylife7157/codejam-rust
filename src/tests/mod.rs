mod gcj2008;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use ::utils::scan::*;
use ::problems::*;
use ::utils::*;

const QR: &str = "qr";

const A: &str = "a";
const B: &str = "b";
const C: &str = "c";

pub fn test(year: &str, round: &str, problem: &str) {
    if let Some(solve_next) = get_solution(&year.to_string(), &round.to_string(), &problem.to_string()) {
        let input = File::open(format!("test_cases/gcj20{}/{}/{}-large-practice.in", year, round, problem.to_uppercase())).expect("");
        let output = File::open(format!("test_cases/gcj20{}/{}/{}-large-practice.out", year, round, problem.to_uppercase())).expect("");
        let mut input_scanner = Scanner::from(Input::File(BufReader::new(input)));
        let mut output_scanner = Scanner::from(Input::File(BufReader::new(output)));

        let t: usize = input_scanner.next_number();
        let result: Vec<String> =
            (1..t+1)
                .map(|i| format!("Case #{:?}: {}", i, solve_next(&mut input_scanner)))
                .collect();

        assert_eq!(result.join("\n"), output_scanner.all_lines());
    } else {
        assert!(false, "Couldn't get solution");
    }
}
