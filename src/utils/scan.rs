use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::str::FromStr;

pub enum Input {
    Stdin,
    File(BufReader<File>)
}

pub struct Scanner {
    stdin: ::std::io::Stdin,
    source: Input,
    buf: String
}

impl From<Input> for Scanner {
    fn from(input: Input) -> Scanner {
        Scanner {
            stdin: ::std::io::stdin(),
            source: input,
            buf: String::new()
        }
    }
}

impl Scanner {
    pub fn all_lines(&mut self) -> String {
        self.buf.clear();
        match self.source {
            Input::Stdin => self.stdin.lock().read_to_string(&mut self.buf).unwrap(),
            Input::File(ref mut reader) => reader.read_to_string(&mut self.buf).unwrap()
        };
        self.buf.trim().to_string()
    }

    pub fn next_line(&mut self) -> String {
        self.buf.clear();
        match self.source {
            Input::Stdin => self.stdin.lock().read_line(&mut self.buf).unwrap(),
            Input::File(ref mut reader) => reader.read_line(&mut self.buf).unwrap()
        };
        self.buf.trim().to_string()
    }

    pub fn next_n_lines(&mut self, n: usize) -> Vec<String> {
        vec![0; n].iter()
            .map(|_| self.next_line())
            .collect()
    }

    pub fn next_number<T>(&mut self) -> T where T: FromStr, <T as FromStr>::Err: ::std::fmt::Debug {
        self.next_line()
            .parse()
            .unwrap()
    }

    pub fn next_numbers<T>(&mut self) -> Vec<T> where T: FromStr, <T as FromStr>::Err: ::std::fmt::Debug {
        self.next_line()
            .split(" ")
            .map(|s| s.parse().unwrap())
            .collect()
    }
}
