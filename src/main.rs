mod problems;
mod utils;

use utils::scan::*;
use problems::get_solution;

fn main() {
    let args: Vec<String> = ::std::env::args().collect();

    if args.len() == 4 {
        let year = &args[1];
        let round = &args[2];
        let problem = &args[3];
        let mut scanner = Scanner::from(Input::Stdin);

        if let Some(solve_next) = get_solution(&year, &round, &problem) {
            let num_of_test_cases: usize = scanner.next_number();

            for i in 1..num_of_test_cases+1 {
                println!("Case #{}: {}", i, solve_next(&mut scanner));
            }
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
