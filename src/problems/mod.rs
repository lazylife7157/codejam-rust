pub mod gcj2008;

use ::utils::*;
use ::utils::scan::*;

pub fn get_solution(year: &String, round: &String, problem: &String) -> Option<fn(&mut Scanner) -> String> {
    match year.as_str() {
        "08" => match round.as_str() {
            "qr" => match problem.as_str() {
                "a" => Some(|scanner| gcj2008::qr::A::from(scanner).solve()),
                "b" => Some(|scanner| gcj2008::qr::B::from(scanner).solve()),
                "c" => Some(|scanner| gcj2008::qr::C::from(scanner).solve()),
                _ => None
            },
            _ => None
        },
        _ => None
    }
}
