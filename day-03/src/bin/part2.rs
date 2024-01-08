fn main() {
    let input = include_str!("./input3.txt");
    let output = part2(input);
    dbg!(output);
}

#[derive(Debug, Clone)]
struct Grid {
    pub data: Vec<Vec<char>>,
    pub width: usize,
    pub height: usize,
}

impl Grid {
    fn new(s: &str) -> Self {
        let mut data: Vec<Vec<char>> = Vec::new();
        for line in s.lines() {
            data.push(line.chars().collect::<Vec<char>>());
        }

        let height = data.len();
        let width = data[0].len();

        Self {
            data,
            height,
            width,
        }
    }

    fn symbols(&self) -> SymbolsIter {
        SymbolsIter::new(self)
    }
}

struct SymbolsIter<'a> {
    grid: &'a Grid,
    next_index: usize,
}

impl<'a> SymbolsIter<'a> {
    fn new(grid: &'a Grid) -> Self {
        Self {
            grid,
            next_index: 0,
        }
    }
}

impl<'a> Iterator for SymbolsIter<'a> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let idx = self.next_index;
            if idx >= self.grid.width * self.grid.height {
                return None;
            }

            let (x, y) = (idx % self.grid.width, idx / self.grid.width);
            self.next_index += 1;
            match self.grid.data[y][x] {
                '0'..='9' | '.' => {}
                _ => {
                    return Some((y, x));
                }
            }
        }
    }
}

fn number_at(grid: &Grid, y: usize, x:usize) -> u32 {
    let mut start: usize = x;
    assert!(grid.data[y][x].is_ascii_digit());

    while start > 0 && grid.data[y][start - 1].is_ascii_digit() {
        start -= 1;
    }
    let mut end: usize = x;
    while end < grid.width -1 && grid.data[y][end+1].is_ascii_digit() {
        end += 1;
    }

    let mut n: u32 = 0;
    for x in start..=end {
        n *= 10;
        n += grid.data[y][x].to_digit(10).unwrap();
    }

    n
}

fn part2(input: &str) -> u32 {
    let grid: Grid = Grid::new(&input);
    let mut secret: u32 = 0;
    for (y, x) in grid.symbols() {
        if grid.data[y][x] != '*' {
            continue;
        }

        let mut number_positions: Vec<(usize, usize)> = Vec::new();

        if y > 0 {
            let above_left: usize = (x > 0 && grid.data[y-1][x-1].is_ascii_digit()) as usize;
            let above: usize = grid.data[y-1][x].is_ascii_digit() as usize;
            let above_right: usize = (x < grid.width -1 && grid.data[y-1][x+1].is_ascii_digit()) as usize;

            let sum: usize = above_left + above + above_right;

            if sum == 0 {
                continue;
            }

            if sum == 2 {
                if above == 0 {
                    number_positions.push((y - 1, x - 1));
                    number_positions.push((y - 1, x + 1));
                } else {
                    number_positions.push((y - 1, x));
                }
            } else if sum == 3 || above == 1 {
                number_positions.push((y - 1, x));
            } else if above_left == 1 {
                number_positions.push((y - 1, x - 1));
            } else {
                assert_eq!(above_right, 1);
                number_positions.push((y - 1, x + 1));
            }
        }

        if x > 0 && grid.data[y][x-1].is_ascii_digit() {
            number_positions.push((y, x - 1));
        }

        if x < grid.width && grid.data[y][x+1].is_ascii_digit() {
            number_positions.push((y, x + 1));
        }

        if y < grid.height - 1 {
            let below_left: usize = (x > 0 && grid.data[y+1][x-1].is_ascii_digit()) as usize;
            let below: usize = grid.data[y + 1][x].is_ascii_digit() as usize;
            let below_right: usize = (x < grid.width - 1 && grid.data[y + 1][x + 1].is_ascii_digit()) as usize;

            let sum: usize = below_left + below + below_right;

            if sum == 0 {
                continue;
            }

            if sum == 2 {
                if below == 0 {
                    number_positions.push((y + 1, x - 1));
                    number_positions.push((y + 1, x + 1));
                } else {
                    number_positions.push((y + 1, x));
                }
            } else if sum == 3 || below == 1 {
                number_positions.push((y + 1, x));
            } else if below_left == 1 {
                number_positions.push((y + 1, x - 1));
            } else {
                assert_eq!(below_right, 1);
                number_positions.push((y + 1, x + 1));
            }
        }

        if number_positions.len() == 2 {
            secret += number_positions.iter().fold(1, |acc, (y,x)| acc * number_at(&grid, *y, *x));
        }
    }

    secret
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = part2(
"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..");
        assert_eq!(result, 467835);
    }
}
