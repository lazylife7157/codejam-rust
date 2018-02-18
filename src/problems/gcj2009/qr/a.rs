use ::problems::*;

pub struct A {
    l: usize,
    d: usize,
    words: Vec<String>,
    pattern: String
}

impl Solution for A {
    fn problems(input: &mut Input) -> Vec<Box<Self>> {
        let ldn: Vec<usize> = input.next_numbers();
        let words = input.next_n_lines(ldn[1]);
        (0..ldn[2])
            .map(|_| Self {
                l: ldn[0],
                d: ldn[1],
                words: words.clone(),
                pattern: input.next_line()
            }).map(Box::new).collect()
    }

    fn solve(&self) -> String {
        let mut pattern = Vec::new();
        let mut sub_pattern = Vec::new();
        let mut is_sub_pattern = false;
        for ch in self.pattern.chars() {
            match ch {
                '(' => is_sub_pattern = true,
                ')' => {
                    is_sub_pattern = false;
                    sub_pattern.sort();
                    pattern.push(sub_pattern.clone());
                    sub_pattern.clear();
                },
                _  if is_sub_pattern => sub_pattern.push(ch),
                _  => pattern.push(vec![ch]),
            }
        }

        let is_matched = |word: &String|
            word.chars().enumerate()
                .find(|&(i, ch)| pattern[i].binary_search(&ch).is_err())
                .is_none();

        self.words.clone().into_iter()
            .filter(is_matched)
            .count()
            .to_string()
    }
}
