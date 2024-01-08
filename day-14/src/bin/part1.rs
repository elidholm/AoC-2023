fn main() {
    let input = include_str!("./input14.txt");
    let output = part1(input);
    dbg!(output);
}

type Grid = Vec<Vec<char>>;

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

fn part1(input: &str) -> usize {
    let mut platform: Platform = Platform::parse(input);
    platform.tilt_north();

    platform.get_load()
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = part1("O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....");
            assert_eq!(result, 136);
    }
}
