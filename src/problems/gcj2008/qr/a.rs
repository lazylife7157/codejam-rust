use ::problems::*;
use std::collections::HashSet;

pub struct A {
    s: usize,
    // search_engines: Vec<String>,
    // q: usize,
    queries: Vec<String>
}

impl Solution for A {
    fn problems(input: &mut Input) -> Vec<Box<Self>> {
        (0..input.next_number())
            .map(|_| {
                let s = input.next_number();
                // let search_engines = input.next_n_lines(s);
                input.skip_n_lines(s);
                let q = input.next_number();
                let queries = input.next_n_lines(q);

                A {
                    s: s,
                    // search_engines: search_engines,
                    // q: q,
                    queries: queries
                }
            }).map(Box::new).collect()
    }

    fn solve(&self) -> String {
        let queries = self.queries.as_slice();
        let mut searched_engines = HashSet::new();
        self.get_min_switch(queries, &mut searched_engines).to_string()
    }
}

impl A {
    fn get_min_switch(&self, queries: &[String], searched_engines: &mut HashSet<String>) -> u32 {
        match queries.len() {
            0 => 0,
            _ => {
                searched_engines.insert(queries[0].clone());
                if searched_engines.len() == self.s {
                    searched_engines.clear();
                    searched_engines.insert(queries[0].clone());
                    1 + self.get_min_switch(&queries[1 ..], searched_engines)
                } else {
                    self.get_min_switch(&queries[1 ..], searched_engines)
                }
            }
        }
    }
 }
