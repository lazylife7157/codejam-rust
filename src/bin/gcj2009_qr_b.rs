extern crate codejam;

use codejam::utils::prelude::*;

const EMPTY: char = 255_u8 as char;

fn main() {
    let mut input = Scanner::from(Source::Stdin);
    let t = input.next_number();
    let result = (0..t)
        .map(|_| next_case(&mut input))
        .map(solve)
        .enumerate()
        .map(format_multi_line)
        .collect::<Vec<String>>()
        .join("\n");

    println!("{}", result);
}

fn solve(case: Case) -> String {
    let mut basins = vec![vec![EMPTY; case.w]; case.h];
    let mut label = 'a' as u8 - 1;

    for i in 0..case.h {
        for j in 0..case.w {
            if basins[i][j] == EMPTY {
                basins[i][j] = flow(&case, i, j, &mut label, &mut basins);
            }
        }
    }

    let result: Vec<String> = basins.iter()
        .map(|x| x.iter().map(|b| b.to_string()).collect::<Vec<String>>().join(" "))
        .collect();

    result.join("\n")
}

fn flow(case: &Case, i: usize, j: usize, label: &mut u8, basins: &mut Vec<Vec<char>>) -> char {
    let current_altitude = case.map[i][j];
    let (altitude, fi, fj) = lowest_cell(case, i, j);
    if altitude < current_altitude {
        if basins[fi][fj] == EMPTY {
            basins[fi][fj] = flow(case, fi, fj, label, basins);
            basins[fi][fj]
        } else {
            basins[fi][fj]
        }
    } else {
        *label += 1;
        *label as char
    }
}

fn lowest_cell(case: &Case, i: usize, j: usize) -> (u32, usize, usize) {
    let mut cells = Vec::new();

    if i > 0 {
        cells.push((case.map[i-1][j], i-1, j));
    }
    if j > 0 {
        cells.push((case.map[i][j-1], i, j-1));
    }
    if j < case.w - 1 {
        cells.push((case.map[i][j+1], i, j+1));
    }
    if i < case.h - 1 {
        cells.push((case.map[i+1][j], i+1, j));
    }

    *cells.iter().min().unwrap_or(&(case.map[i][j], i, j))
}

struct Case {
    h: usize,
    w: usize,
    map: Vec<Vec<u32>>
}

fn next_case(input: &mut Scanner) -> Case {
    let hw = input.next_numbers();
    let map = input.next_n_lines(hw[0])
        .iter()
        .map(row_to_cells)
        .collect();

    Case {
        h: hw[0],
        w: hw[1],
        map: map
    }
}

fn row_to_cells(row: &String) -> Vec<u32> {
    row.split_whitespace()
        .map(|cell| cell.parse().unwrap())
        .collect()
}
