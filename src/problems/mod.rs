pub mod gcj2008;
pub mod gcj2009;

use ::utils::scan::Scanner;

type Input = Scanner;
type Output = Vec<String>;

pub trait Problem {
    fn problems(&mut Input) -> Vec<Box<Self>>;
    fn solve(&self) -> String;
}

fn solve_all<T>(problems: Vec<Box<T>>) -> Option<Output> where T: Problem {
    Some(
        problems
            .into_iter()
            .enumerate()
            .map(solve_and_format)
            .collect()
    )
}

fn solve_and_format<T>(x: (usize, Box<T>)) -> String where T: Problem {
    let case_num = x.0 + 1;
    let problem = x.1;
    format!("Case #{}: {}", case_num, problem.solve())
}

pub fn solve(year: &String, round: &String, problem: &String, input: &mut Input) -> Option<Output> {
    match year.as_str() {
        "08" =>
            match round.as_str() {
                "qr" => {
                    use self::gcj2008::qr::*;
                    match problem.as_str() {
                        "a" => solve_all(A::problems(input)),
                        "b" => solve_all(B::problems(input)),
                        "c" => solve_all(C::problems(input)),
                        _ => None
                    }
                },
            _ => None
        },
        "09" =>
            match round.as_str() {
                "qr" => {
                    use self::gcj2009::qr::*;
                    match problem.as_str() {
                        "a" => solve_all(A::problems(input)),
                        _ => None
                    }
                },
            _ => None
        },
        _ => None
    }
}
