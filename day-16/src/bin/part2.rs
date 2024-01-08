use std::collections::HashSet;

fn main() {
    let input = include_str!("./input16.txt");
    let output = part2(input);
    dbg!(output);
}

type Room = Vec<Vec<char>>;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Beam {
    pub x: usize,
    pub y: usize,
    pub dir: Direction,
}

impl Beam {
    fn new(x: usize, y: usize, dir: Direction) -> Self {
        Beam { x, y, dir }
    }

    fn get_next(&self, x_max: usize, y_max: usize) -> Option<(usize, usize)> {
        match self.dir {
            Direction::Up => {
                if self.y == 0 {
                    None
                } else {
                    Some((self.x, self.y - 1))
                }
            }
            Direction::Down => {
                if self.y == y_max {
                    None
                } else {
                    Some((self.x, self.y + 1))
                }
            }
            Direction::Left => {
                if self.x == 0 {
                    None
                } else {
                    Some((self.x - 1, self.y))
                }
            }
            Direction::Right => {
                if self.x == x_max {
                    None
                } else {
                    Some((self.x + 1, self.y))
                }
            }
        }
    }
}

fn reflect(dir: Direction, reflector: char) -> Direction {
    match reflector {
        '\\' => {
            match dir {
                Direction::Up => {
                    Direction::Left
                }
                Direction::Down => {
                    Direction::Right
                }
                Direction::Left => {
                    Direction::Up
                }
                Direction::Right => {
                    Direction::Down
                }
            }
        }
        '/' => {
            match dir {
                Direction::Up => {
                    Direction::Right
                }
                Direction::Down => {
                    Direction::Left
                }
                Direction::Left => {
                    Direction::Down
                }
                Direction::Right => {
                    Direction::Up
                }
            }
        }
        _ => {
            panic!("Not a reflector: {}", reflector);
        }
    }
}

fn split(dir: Direction, splitter: char) -> Option<(Direction, Direction)> {
    match splitter {
        '|' => {
            match dir {
                Direction::Left => {
                    Some((Direction::Up, Direction::Down))
                }
                Direction::Right => {
                    Some((Direction::Up, Direction::Down))
                }
                _ => None
            }
        }
        '-' => {
            match dir {
                Direction::Up => {
                    Some((Direction::Left, Direction::Right))
                }
                Direction::Down => {
                    Some((Direction::Left, Direction::Right))
                }
                _ => None
            }
        }
        _ => {
            panic!("Not a splitter: {}", splitter);
        }
    }
}

fn insert_beam(beams: &mut Vec<Beam>, beam: Beam, beams_cache: &mut HashSet<Beam>, energized: &mut HashSet<(usize, usize)>) {
    if !beams_cache.contains(&beam) {
        beams.push(beam.clone());
        beams_cache.insert(beam.clone());
        energized.insert((beam.x, beam.y));
    }
}

fn propagate_beam(room: Room, start: (usize, usize), start_dir: Direction) -> usize {
    let n_rows: usize = room.len();
    let n_cols: usize = room[0].len();

    let mut beams: Vec<Beam> = Vec::new();
    let mut energized: HashSet<(usize, usize)> = HashSet::new();
    let mut beams_cache: HashSet<Beam> = HashSet::new();

    let start_char: char = room[start.1][start.0];
    match start_char {
        '\\'|'/' => {
            let dir = reflect(start_dir, start_char);
            insert_beam(&mut beams, Beam::new(start.0, start.1, dir), &mut beams_cache, &mut energized);
        }
        '-'|'|' => {
            if let Some((dir1, dir2)) = split(start_dir.clone(), start_char) {
                insert_beam(&mut beams, Beam::new(start.0, start.1, dir1), &mut beams_cache, &mut energized);
                insert_beam(&mut beams, Beam::new(start.0, start.1, dir2), &mut beams_cache, &mut energized);
            } else {
                insert_beam(&mut beams, Beam::new(start.0, start.1, start_dir), &mut beams_cache, &mut energized)
            }
        }
        _ => {
            insert_beam(&mut beams, Beam::new(start.0, start.1, start_dir), &mut beams_cache, &mut energized);
        }
    }

    while beams.len() > 0 {
        let beam = beams.pop().unwrap();
        if let Some((next_x, next_y)) = beam.get_next(n_cols - 1, n_rows - 1) {
            match room[next_y][next_x] {
                '\\'|'/' => {
                    let next_dir = reflect(beam.dir, room[next_y][next_x]);
                    insert_beam(&mut beams, Beam::new(next_x, next_y, next_dir), &mut beams_cache, &mut energized);
                }
                '-'|'|' => {
                    if let Some((dir1, dir2)) = split(beam.dir.clone(), room[next_y][next_x]) {
                        insert_beam(&mut beams, Beam::new(next_x, next_y, dir1), &mut beams_cache, &mut energized);
                        insert_beam(&mut beams, Beam::new(next_x, next_y, dir2), &mut beams_cache, &mut energized);
                    } else {
                        insert_beam(&mut beams, Beam::new(next_x, next_y, beam.dir), &mut beams_cache, &mut energized)
                    }
                }
                _ => {
                    insert_beam(&mut beams, Beam::new(next_x, next_y, beam.dir), &mut beams_cache, &mut energized);
                }
            }
        }
    }

    energized.len()
}

fn part2(input: &str) -> usize {
    let mut secret: usize = 0;
    let room: Room = input.lines().map(|l| l.chars().collect()).collect();
    let n_rows: usize = room.len();
    let n_cols: usize = room[0].len();

    for x_start in 0..n_cols {
        let mut start: (usize, usize) = (x_start, 0);
        let mut result = propagate_beam(room.clone(), start, Direction::Down);
        if result > secret {
            secret = result;
        }

        start = (x_start, n_rows - 1);
        result = propagate_beam(room.clone(), start, Direction::Up);
        if result > secret {
            secret = result;
        }
    }

    for y_start in 0..n_rows {
        let mut start: (usize, usize) = (0, y_start);
        let mut result = propagate_beam(room.clone(), start, Direction::Right);
        if result > secret {
            secret = result;
        }

        start = (n_cols - 1, y_start);
        result = propagate_beam(room.clone(), start, Direction::Left);
        if result > secret {
            secret = result;
        }
    }

    secret
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = part2(r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....");
        assert_eq!(result, 51);
    }
}
