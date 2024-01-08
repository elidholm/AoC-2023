use std::collections::HashMap;

fn main() {
    let input = include_str!("./input14.txt");
    let output = part2(input, 1000000000);
    dbg!(output);
}

type Grid = Vec<Vec<char>>;

#[derive(Debug, Clone)]
struct Platform {
    pub grid: Grid,
    pub height: usize,
    pub width: usize,
}

impl Platform {
    fn parse(input: &str) -> Self {
        let mut grid: Grid = Vec::new();
        for line in input.lines() {
            let mut row: Vec<char> = Vec::new();
            for c in line.chars() {
                row.push(c);
            }
            grid.push(row);
        }
        let height = grid.len();
        let width = grid[0].len();
        Self { grid, height, width }
    }

    fn tilt_north(&mut self) {
        for i in 1..self.height {
            for j in 0..self.width {
                if self.grid[i][j] == 'O' {
                    let mut k = 0;
                    while (i - k) > 0 && self.grid[i - k - 1][j] == '.' {
                        k += 1;
                    }

                    if k > 0 {
                        self.grid[i][j] = '.';
                        self.grid[i - k][j] = 'O';
                    }
                }
            }
        }
    }

    fn tilt_west(&mut self) {
        for i in 0..self.height {
            for j in 1..self.width {
                if self.grid[i][j] == 'O' {
                    let mut k = 0;
                    while (j - k) > 0 && self.grid[i][j - k - 1] == '.' {
                        k += 1;
                    }

                    if k > 0 {
                        self.grid[i][j] = '.';
                        self.grid[i][j - k] = 'O';
                    }
                }
            }
        }
    }

    fn tilt_south(&mut self) {
        for i in (0..(self.height-1)).rev() {
            for j in 0..self.width {
                if self.grid[i][j] == 'O' {
                    let mut k = 0;
                    while (i + k + 1) < self.height && self.grid[i + k + 1][j] == '.' {
                        k += 1;
                    }

                    if k > 0 {
                        self.grid[i][j] = '.';
                        self.grid[i + k][j] = 'O';
                    }
                }
            }
        }
    }

    fn tilt_east(&mut self) {
        for i in 0..self.height {
            for j in (0..(self.width - 1)).rev() {
                if self.grid[i][j] == 'O' {
                    let mut k = 0;
                    while (j + k + 1) < self.grid[i].len() && self.grid[i][j + k + 1] == '.' {
                        k += 1;
                    }

                    if k > 0 {
                        self.grid[i][j] = '.';
                        self.grid[i][j + k] = 'O';
                    }
                }
            }
        }
    }

    fn spin_cycle(&mut self) {
        self.tilt_north();
        self.tilt_west();
        self.tilt_south();
        self.tilt_east();
    }

    fn get_load(&self) -> usize {
        let mut load: usize = 0;
        for i in 0..self.height {
            for j in 0..self.width {
                if self.grid[i][j] == 'O' {
                    load += self.height - i;
                }
            }
        }

        load
    }
}

fn part2(input: &str, n_cycles: usize) -> usize {
    let mut platform: Platform = Platform::parse(input);

    let mut state_cache: HashMap<Grid, usize> = HashMap::new();
    state_cache.insert(platform.grid.clone(), 0);

    for i in 1..=n_cycles {
        platform.spin_cycle();
        if let Some(cycle_start) = state_cache.get(&platform.grid) {
            let cycle_length = i - cycle_start;
            let cycle_offset = n_cycles - cycle_start;
            let cycle_index = cycle_offset % cycle_length;
            let end_state = cycle_start + cycle_index;
            platform.grid = state_cache.iter().find(|(_, &v)| v == end_state).unwrap().0.clone();
            break;
        } else {
            state_cache.insert(platform.grid.clone(), i);
        }
    }

    platform.get_load()
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = part2("O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....", 1000000000);
        assert_eq!(result, 64);
    }
}
