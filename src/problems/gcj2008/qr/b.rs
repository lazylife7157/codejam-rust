use ::utils::Problem;
use ::utils::scan::Scanner;

pub struct B {
    t: u32,
    na: usize,
    nb: usize,
    dep_a_to_b: Vec<u32>,
    arr_a_to_b: Vec<u32>,
    dep_b_to_a: Vec<u32>,
    arr_b_to_a: Vec<u32>
}

impl<'a> From<&'a mut Scanner> for B {
    fn from(scanner: &'a mut Scanner) -> B {
        let t = scanner.next_number();
        let ns = scanner.next_numbers();
        let na = ns[0];
        let nb = ns[1];

        let parse = |s: &String| {
            let temp: Vec<String> = s.split(":").map(String::from).collect();
            temp[0].parse().unwrap_or(0) * 60 + temp[1].parse().unwrap_or(0)
        };

        let mut get_timetables = |n| {
            let mut dep = Vec::new();
            let mut arr = Vec::new();
            for line in scanner.next_n_lines(n) {
                let times: Vec<String> = line.split(" ").map(String::from).collect();
                dep.push(parse(&times[0]));
                arr.push(parse(&times[1]));
            }
            dep.sort();
            arr.sort();
            (dep, arr)
        };

        let (dep_a_to_b, arr_a_to_b) = get_timetables(na);
        let (dep_b_to_a, arr_b_to_a) = get_timetables(nb);

        B {
            t: t,
            na: na,
            nb: nb,
            dep_a_to_b: dep_a_to_b,
            arr_a_to_b: arr_a_to_b,
            dep_b_to_a: dep_b_to_a,
            arr_b_to_a: arr_b_to_a
        }
    }
}

impl Problem for B {
    fn solve(&self) -> String {
        let start_at_a = self.na - self.get_count_of_unnecessary_departures(&self.dep_a_to_b.as_slice(), &self.arr_b_to_a.as_slice());
        let start_at_b = self.nb - self.get_count_of_unnecessary_departures(&self.dep_b_to_a.as_slice(), &self.arr_a_to_b.as_slice());
        format!("{} {}", start_at_a, start_at_b)
    }
}

impl B {
    fn get_count_of_unnecessary_departures(&self, dep: &[u32], arr: &[u32]) -> usize {
        match dep.len() {
            0 => 0,
            _ => {
                if arr.len() > 0 && dep[0] >= arr[0] + self.t {
                    1 + self.get_count_of_unnecessary_departures(&dep[1..], &arr[1..])
                } else {
                    self.get_count_of_unnecessary_departures(&dep[1..], arr)
                }
            }
        }
    }
}
