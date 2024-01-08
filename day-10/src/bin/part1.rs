use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let input = include_str!("./input10.txt");
    let output = part1(input);
    dbg!(output);
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Position(u32, u32);

fn build_grid(input: &str) -> (HashMap<Position, char>, Position) {
    let mut grid: HashMap<Position, char> = HashMap::new();
    let mut start: Position = Position(0,0);
    for (row, line) in input.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            grid.insert(Position(row as u32, col as u32), ch);
            if ch == 'S' {
                start = Position(row as u32, col as u32);
            }
        }
    }
    (grid, start)
}

fn build_loop(grid: &HashMap<Position, char>, start: &Position) -> HashSet<Position> {
    let mut queue: VecDeque<Position> = VecDeque::new();
    let mut pipe_loop: HashSet<Position> = HashSet::new();
    let n_rows: u32 = grid.keys().map(|k| k.0).max().unwrap();
    let n_cols: u32 = grid.keys().map(|k| k.1).max().unwrap();

    queue.push_back(*start);
    while !queue.is_empty() {
        let pos: Position = queue.pop_front().unwrap();
        pipe_loop.insert(pos);
        let pipe: &char = grid.get(&pos).unwrap();
        if pos.0 > 0 && ['S', '|', 'L', 'J'].contains(pipe) {
            let north: Position = Position(pos.0 - 1, pos.1);
            let north_pipe: &char = grid.get(&north).unwrap();
            if ['|', '7', 'F'].contains(north_pipe) && !pipe_loop.contains(&north) {
                queue.push_back(north);
            }
        }
        if pos.0 < n_rows && ['S', '|', '7', 'F'].contains(pipe) {
            let south: Position = Position(pos.0 + 1, pos.1);
            let south_pipe: &char = grid.get(&south).unwrap();
            if ['|', 'L', 'J'].contains(south_pipe) && !pipe_loop.contains(&south) {
                queue.push_back(south);
            }
        }
        if pos.1 > 0 && ['S', '-', '7', 'J'].contains(pipe) {
            let west: Position = Position(pos.0, pos.1 - 1);
            let west_pipe: &char = grid.get(&west).unwrap();
            if ['-', 'L', 'F'].contains(west_pipe) && !pipe_loop.contains(&west) {
                queue.push_back(west);
            }
        }
        if pos.1 < n_cols && ['S', '-', 'L', 'F'].contains(pipe) {
            let east: Position = Position(pos.0, pos.1 + 1);
            let east_pipe: &char = grid.get(&east).unwrap();
            if ['-', 'J', '7'].contains(east_pipe) && !pipe_loop.contains(&east) {
                queue.push_back(east);
            }
        }
    }

    pipe_loop
}

fn part1(input: &str) -> u32 {
    let (grid, start): (HashMap<Position, char>, Position) = build_grid(input);
    let pipe_loop: HashSet<Position> = build_loop(&grid, &start);

    pipe_loop.len() as u32 / 2
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = part1("-L|F7
7S-7|
L|7||
-L-J|
L|-JF");
        assert_eq!(result, 4);
    }

    #[test]
    fn it_still_works() {
        let result = part1("7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ");
        assert_eq!(result, 8);
    }
}
