use std::io::prelude::*;
use std::str::FromStr;

pub trait Scan {
    fn next_line(&mut self) -> Option<String>;

    fn next_n_lines(&mut self, n: usize) -> Vec<String> {
        (0..n).filter_map(|_| self.next_line()).collect()
    }

    fn all_lines(&mut self) -> Vec<String> {
        let mut lines = Vec::new();
        while let Some(line) = self.next_line() {
            lines.push(line);
        }
        lines
    }

    fn skip_line(&mut self) {
        self.next_line();
    }

    fn skip_n_lines(&mut self, n: usize) {
        for _ in 0..n {
            self.next_line();
        }
    }

    fn next_line_as_number<T>(&mut self) -> Option<T> where T: FromStr {
        self.next_line().and_then(|x| x.trim().parse().ok())
    }

    fn next_line_as_numbers<T>(&mut self) -> Vec<T> where T: FromStr {
        self.next_line()
            .unwrap_or(String::new())
            .split_whitespace()
            .filter_map(|x| x.trim().parse().ok())
            .collect()
    }
}

const BUF_SIZE: usize = 1024;
const NEW_LINE: u8 = '\n' as u8;
pub struct Scanner<T> where T: Read {
    i: usize,
    buf: [u8; BUF_SIZE],
    reader: T
}

impl<T> From<T> for Scanner<T> where T: Read {
    fn from(reader: T) -> Self {
        Self {
            i: BUF_SIZE - 1,
            buf: [0; BUF_SIZE],
            reader: reader
        }
    }
}

impl<'a, T> Scan for Scanner<T> where T: Read {
    fn next_line(&mut self) -> Option<String> {
        let mut buf = Vec::new();

        loop {
            self.i += 1;
            if self.i == BUF_SIZE {
                self.i = 0;
                match self.reader.read(&mut self.buf) {
                    Ok(n) if n > 0 => (),
                    _ => break
                }
            }

            if self.buf[self.i] == NEW_LINE {
                break
            } else {
                buf.push(self.buf[self.i])
            }
        }

        String::from_utf8(buf).ok()
    }
}
