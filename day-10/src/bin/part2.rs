use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let input = include_str!("./input10.txt");
    let output = part2(input);
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

fn build_loop(grid: &HashMap<Position, char>, start: &Position) -> (HashSet<Position>, HashMap<Position, char>) {
    let mut queue: VecDeque<Position> = VecDeque::new();
    let mut pipe_loop: HashSet<Position> = HashSet::new();
    let mut pipe_types: HashSet<char> = HashSet::from(['|', '-', 'L', 'J', '7', 'F']);
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
                if *pipe == 'S' {
                    pipe_types = pipe_types.intersection(&HashSet::from(['|', 'L', 'J'])).cloned().collect();
                }
            }
        }
        if pos.0 < n_rows && ['S', '|', '7', 'F'].contains(pipe) {
            let south: Position = Position(pos.0 + 1, pos.1);
            let south_pipe: &char = grid.get(&south).unwrap();
            if ['|', 'L', 'J'].contains(south_pipe) && !pipe_loop.contains(&south) {
                queue.push_back(south);
                if *pipe == 'S' {
                    pipe_types = pipe_types.intersection(&HashSet::from(['|', '7', 'F'])).cloned().collect();
                }
            }
        }
        if pos.1 > 0 && ['S', '-', '7', 'J'].contains(pipe) {
            let west: Position = Position(pos.0, pos.1 - 1);
            let west_pipe: &char = grid.get(&west).unwrap();
            if ['-', 'L', 'F'].contains(west_pipe) && !pipe_loop.contains(&west) {
                queue.push_back(west);
                if *pipe == 'S' {
                    pipe_types = pipe_types.intersection(&HashSet::from(['-', '7', 'J'])).cloned().collect();
                }
            }
        }
        if pos.1 < n_cols && ['S', '-', 'L', 'F'].contains(pipe) {
            let east: Position = Position(pos.0, pos.1 + 1);
            let east_pipe: &char = grid.get(&east).unwrap();
            if ['-', 'J', '7'].contains(east_pipe) && !pipe_loop.contains(&east) {
                queue.push_back(east);
                if *pipe == 'S' {
                    pipe_types = pipe_types.intersection(&HashSet::from(['-', 'L', 'F'])).cloned().collect();
                }
            }
        }
    }
    let start_pipe = pipe_types.iter().next().unwrap();
    let mut new_grid = grid.clone();
    new_grid.insert(*start, *start_pipe);
    (pipe_loop, new_grid)
}

fn count_interior(pipe_loop: &HashSet<Position>, grid: &HashMap<Position, char>) -> u32 {
    let mut count: u32 = 0;
    let mut new_grid: HashMap<Position, char> = HashMap::new();
    let n_rows = grid.keys().map(|k| k.0).max().unwrap();
    let n_cols = grid.keys().map(|k| k.1).max().unwrap();
    for pos in grid.keys() {
        if !pipe_loop.contains(pos) {
            new_grid.insert(*pos, '.');
        } else {
            let pipe: &char = grid.get(pos).unwrap();
            new_grid.insert(*pos, *pipe);
        }
    }
    for row in 0..=n_rows {
        for col in 0..=n_cols {
            let pos: Position = Position(row, col);
            let pipe: &char = new_grid.get(&pos).unwrap();
            if *pipe != '.' {
                continue;
            }
            let mut intersections: u32 = 0;
            let mut corner_pipes: VecDeque<char> = VecDeque::new();
            for i in (col + 1)..=n_cols {
                let scan_pos: Position = Position(row, i);
                let scan_pipe: &char = new_grid.get(&scan_pos).unwrap();
                if *scan_pipe == '|' {
                    intersections += 1;
                } else if ['F', 'L'].contains(scan_pipe) {
                    corner_pipes.push_back(*scan_pipe);
                } else if !corner_pipes.is_empty()
                    && ((*scan_pipe == 'J' && *corner_pipes.iter().last().unwrap() == 'F')
                        || (*scan_pipe == '7' && *corner_pipes.iter().last().unwrap() == 'L')) {
                    let _ = corner_pipes.pop_back();
                    intersections += 1;
                }
            }
            if intersections % 2 == 1 {
                count += 1;
            }
        }
    }
    count
}


fn part2(input: &str) -> u32 {
    let (grid, start): (HashMap<Position, char>, Position) = build_grid(input);
    let (pipe_loop, new_grid): (HashSet<Position>, HashMap<Position, char>) = build_loop(&grid, &start);

    count_interior(&pipe_loop, &new_grid)
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = part2("...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........");
        assert_eq!(result, 4);
    }

    #[test]
    fn it_still_works() {
        let result = part2(".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...");
        assert_eq!(result, 8);
    }

    #[test]
    fn it_still_works_omg() {
        let result = part2("FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L");
        assert_eq!(result, 10);
    }
}
