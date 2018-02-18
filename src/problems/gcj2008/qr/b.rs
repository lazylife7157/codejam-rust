use ::problems::*;

pub struct B {
    t: u32,
    na: usize,
    nb: usize,
    dep_a_to_b: Vec<u32>,
    arr_a_to_b: Vec<u32>,
    dep_b_to_a: Vec<u32>,
    arr_b_to_a: Vec<u32>
}

impl Solution for B {
    fn problems(input: &mut Input) -> Vec<Box<Self>> {
        (0..input.next_number())
            .map(|_| {
                let t = input.next_number();
                let ns = input.next_numbers();
                let na = ns[0];
                let nb = ns[1];

                let time_to_min = |s: &String| {
                    let time: Vec<u32> = s.split(":").map(|t| t.parse().unwrap()).collect();
                    time[0] * 60 + time[1]
                };

                let mut get_timetables = |n| {
                    let mut dep = Vec::new();
                    let mut arr = Vec::new();
                    for line in input.next_n_lines(n) {
                        let times: Vec<String> = line.split_whitespace().map(String::from).collect();
                        dep.push(time_to_min(&times[0]));
                        arr.push(time_to_min(&times[1]));
                    }
                    dep.sort();
                    arr.sort();
                    (dep, arr)
                };

                let (dep_a_to_b, arr_a_to_b) = get_timetables(na);
                let (dep_b_to_a, arr_b_to_a) = get_timetables(nb);

                Self {
                    t: t,
                    na: na,
                    nb: nb,
                    dep_a_to_b: dep_a_to_b,
                    arr_a_to_b: arr_a_to_b,
                    dep_b_to_a: dep_b_to_a,
                    arr_b_to_a: arr_b_to_a
                }
            }).map(Box::new).collect()
    }

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
