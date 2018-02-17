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
    let input_file = File::open(format!("test_cases/gcj20{}/{}/{}-large-practice.in", year, round, problem.to_uppercase())).expect("");
    let output_file = File::open(format!("test_cases/gcj20{}/{}/{}-large-practice.out", year, round, problem.to_uppercase())).expect("");

    let mut input_scanner = Scanner::from(Source::File(BufReader::new(input_file)));
    let mut output_scanner = Scanner::from(Source::File(BufReader::new(output_file)));

    if let Some(output) = solve(&year.to_string(), &round.to_string(), &problem.to_string(), &mut input_scanner) {
        assert_eq!(output.join("\n"), output_scanner.all_lines())
    } else {
        assert!(false, "Couldn't get solution");
    }
}
