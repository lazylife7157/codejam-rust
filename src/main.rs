mod problems;
mod utils;

use utils::scan::*;
use problems::solve;

fn main() {
    let args: Vec<String> = ::std::env::args().collect();

    if args.len() == 4 {
        let year = &args[1];
        let round = &args[2];
        let problem = &args[3];

        let mut input = Scanner::from(Source::Stdin);
        if let Some(output) = solve(year, round, problem, &mut input) {
            println!("{}", output.join("\n"));
        } else {
            print_usage();
        }
    } else {
        print_usage();
    }
}

fn print_usage() {
    eprintln!("Usage: codejam [PROBLEM_CODE] < [INPUT_FILE] > [OUTPUT_FILE]");
}
