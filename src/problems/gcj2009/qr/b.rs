use ::problems::*;

pub struct B {
    h: usize,
    w: usize,
    map: Vec<Vec<u32>>,
    empty: char
}

impl Solution for B {
    fn problems(input: &mut Input) -> Vec<Box<Self>> {
        let t: usize = input.next_number();
        (0..t)
            .map(|_| {
                let hw = input.next_numbers();
                let row_to_cells = |row: &String|
                    row.split_whitespace()
                        .map(|cell| cell.parse().unwrap())
                        .collect();
                let map =
                    input.next_n_lines(hw[0])
                        .iter()
                        .map(row_to_cells)
                        .collect();
                Self {
                    h: hw[0],
                    w: hw[1],
                    map: map,
                    empty: 255_u8 as char
                }
            }).map(Box::new).collect()
    }

    fn solve(&self) -> String {
        let mut basins = vec![vec![self.empty; self.w]; self.h];
        let mut label = 'a' as u8 - 1;
        for i in 0..self.h {
            for j in 0..self.w {
                if basins[i][j] == self.empty {
                    basins[i][j] = self.flow(i, j, &mut label, &mut basins);
                }
            }
        }

        let result: Vec<String> = basins.iter()
            .map(|x| x.iter().map(|b| b.to_string()).collect::<Vec<String>>().join(" "))
            .collect();

        format!("\n{}", result.join("\n"))
    }
}

impl B {
    fn flow(&self, i: usize, j: usize, label: &mut u8, basins: &mut Vec<Vec<char>>) -> char {
        let current_altitude = self.map[i][j];
        let (altitude, fi, fj) = self.lowest_cell(i, j);
        if altitude < current_altitude {
            if basins[fi][fj] == self.empty {
                basins[fi][fj] = self.flow(fi, fj, label, basins);
                basins[fi][fj]
            } else {
                basins[fi][fj]
            }
        } else {
            *label += 1;
            *label as char
        }
    }

    fn lowest_cell(&self, i: usize, j: usize) -> (u32, usize, usize) {
        let mut cells = Vec::new();
        if i > 0 {
            cells.push((self.map[i-1][j], i-1, j));
        }
        if j > 0 {
            cells.push((self.map[i][j-1], i, j-1));
        }
        if j < self.w - 1 {
            cells.push((self.map[i][j+1], i, j+1));
        }
        if i < self.h - 1 {
            cells.push((self.map[i+1][j], i+1, j));
        }
        *cells.iter().min().unwrap_or(&(self.map[i][j], i, j))
    }
}
