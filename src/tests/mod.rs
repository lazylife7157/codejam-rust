mod gcj2008;
mod gcj2009;

use std::process::*;
use std::fs::File;
use std::io::prelude::*;

const QR: &str = "qr";

const A: &str = "a";
const B: &str = "b";
const C: &str = "c";

const IN: &str = "in";
const OUT: &str = "out";

fn get_file(year: &str, round: &str, problem: &str, ext: &str) -> File {
    File::open(format!("test_cases/gcj20{}/{}/{}-large-practice.{}", year, round, problem.to_uppercase(), ext)).expect("Couldn't open file")
}

fn get_solver(year: &str, round: &str, problem: &str) -> Child {
    Command::new("cargo")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .arg("run")
        .arg("--bin")
        .arg(format!("gcj20{}_{}_{}", year, round, problem))
        .spawn()
        .expect("Failed to execute process.")
}

pub fn test(year: &str, round: &str, problem: &str) {
    let mut input = Vec::new();
    let mut output = String::new();
    let mut _output = String::new();

    get_file(year, round, problem, IN)
        .read_to_end(&mut input)
        .expect("Failed to read input_file.");

    get_file(year, round, problem, OUT)
        .read_to_string(&mut output)
        .expect("Failed to read output_file.");

    let solver = get_solver(year, round, problem);
    solver.stdin.unwrap().write_all(&input).expect("Failed to write input");
    solver.stdout.unwrap().read_to_string(&mut _output).expect("Failed to read output");

    assert_eq!(output, _output);
}
