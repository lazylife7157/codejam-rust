mod gcj2008;
mod gcj2009;
mod ks2018;

use std::time::SystemTime;
use std::process::*;
use std::fs::File;
use std::io::prelude::*;

const GCJ: &str = "gcj";
const KS: &str = "ks";

const QR: &str = "qr";
const PR: &str = "pr";

const A: &str = "a";
const B: &str = "b";
const C: &str = "c";
const D: &str = "d";

const IN: &str = "in";
const OUT: &str = "out";

fn get_file(contest: &str, year: &str, round: &str, problem: &str, ext: &str) -> File {
    File::open(format!("test_cases/{}20{}/{}/{}-large-practice.{}", contest, year, round, problem.to_uppercase(), ext)).expect("Couldn't open file")
}

fn get_solver(contest:&str, year: &str, round: &str, problem: &str) -> Child {
    Command::new("cargo")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .arg("run")
        .arg("--release")
        .arg("--bin")
        .arg(format!("{}20{}_{}_{}", contest, year, round, problem))
        .spawn()
        .expect("Failed to execute process.")
}

pub fn test(contest: &str, year: &str, round: &str, problem: &str) {
    let mut input = Vec::new();
    let mut output = String::new();
    let mut _output = String::new();

    get_file(contest, year, round, problem, IN)
        .read_to_end(&mut input)
        .expect("Failed to read input_file.");

    get_file(contest, year, round, problem, OUT)
        .read_to_string(&mut output)
        .expect("Failed to read output_file.");

    let solver = get_solver(contest, year, round, problem);

    let now = SystemTime::now();
    solver.stdin.unwrap().write_all(&input).expect("Failed to write input");
    solver.stdout.unwrap().read_to_string(&mut _output).expect("Failed to read output");
    let elapsed = now.elapsed().expect("Failed to elapse.");
    println!("[{}20{}_{}_{}] {}ms", contest, year, round, problem, elapsed.subsec_nanos() / 1000_000);

    assert_eq!(output, _output);
}
