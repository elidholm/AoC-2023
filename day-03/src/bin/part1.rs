fn main() {
    let input = include_str!("./input3.txt");
    let output = part1(input);
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

        let height: usize = data.len();
        let width: usize = data[0].len();

        Self {
            data,
            height,
            width,
        }
    }
}

fn is_symbol(c: char) -> bool {
    match c {
        '0'..='9' | '.' => false,
        _ => true,
    }
}

fn part1(input: &str) -> u32 {
    let grid: Grid = Grid::new(&input);
    let mut secret: u32 = 0;
    for y in 0..grid.height {
        let mut x = 0;
        while x < grid.width {
            match grid.data[y][x] {
                '.' => {},
                '0'..='9' => {
                    let begin = x;
                    while x < grid.width && grid.data[y][x].is_ascii_digit() {
                        x += 1;
                    }

                    let end = x - 1;

                    let before_x  =begin.saturating_sub(1);
                    let after_x = (end + 1).min(grid.width - 1);

                    if (y > 0 && (before_x..=after_x).any(|x| is_symbol(grid.data[y-1][x])))
                        || (y < grid.height -1 && (before_x..=after_x).any(|x| is_symbol(grid.data[y+1][x])))
                        || (begin > 0 && is_symbol(grid.data[y][begin - 1]))
                        || (end < grid.width - 1 && is_symbol(grid.data[y][end + 1]))
                    {
                        let mut n = 0;
                        for x in begin..=end {
                            n *=10;
                            n += grid.data[y][x].to_digit(10).unwrap();
                        }

                        secret += n;
                        continue;
                    }
                },
                _ => {},
            }

            x += 1;
        }
    }

    secret
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = part1("467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..");
        assert_eq!(result, 4361);
    }
}

