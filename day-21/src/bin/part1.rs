use std::collections::HashSet;

fn main() {
    let input = include_str!("./input21.txt");
    let output = part1(input, 64);
    dbg!(output);
}

type Garden = Vec<Vec<char>>;

fn find_start(garden: &Garden) -> Option<(usize, usize)> {
    let n_rows = garden.len();
    let n_cols = garden[0].len();

    for i in 0..n_rows {
        for j in 0..n_cols {
            if garden[i][j] == 'S' {
                return Some((i, j));
            }
        }
    }
    None
}

fn part1(input: &str, n_steps: usize) -> usize {
    let garden: Garden = input.lines().map(|line| line.chars().collect()).collect();
    let n_rows = garden.len();
    let n_cols = garden[0].len();

    let mut positions: HashSet<(usize, usize)> = HashSet::new();

    if let Some((i, j)) = find_start(&garden) {
        positions.insert((i, j));

        for _ in 0..n_steps {
            let mut new_positions: HashSet<(usize, usize)> = HashSet::new();
            for (i, j) in positions {
                if i > 0 && garden[i - 1][j] != '#' {
                    new_positions.insert((i - 1, j));
                }
                if i < n_rows - 1 && garden[i + 1][j] != '#' {
                    new_positions.insert((i + 1, j));
                }
                if j > 0 && garden[i][j - 1] != '#'{
                    new_positions.insert((i, j - 1));
                }
                if j < n_cols - 1 && garden[i][j + 1] != '#' {
                    new_positions.insert((i, j + 1));
                }
            }
            positions = new_positions;
        }
    } else {
        panic!("No starting point found");
    }

    positions.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1("...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........", 6);
        assert_eq!(result, 16);
    }
}
