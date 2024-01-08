use std::collections::HashSet;

fn main() {
    let input = include_str!("./input11.txt");
    let output = part1(input);
    dbg!(output);
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Galaxy {
    pub x: u32,
    pub y: u32,
}

impl Galaxy {
    fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct GalaxyMap {
    pub galaxies: Vec<Galaxy>,
}

impl GalaxyMap {
    fn new(galaxies: Vec<Galaxy>) -> Self {
        Self { galaxies }
    }

    fn add_offset(&mut self) {
        let n_rows = self.galaxies.iter().map(|g| g.x).max().unwrap();
        let n_cols = self.galaxies.iter().map(|g| g.y).max().unwrap();

        let mut empty_rows: HashSet<u32> = (0..=n_rows).collect();
        let mut empty_cols: HashSet<u32> = (0..=n_cols).collect();
        for galaxy in self.galaxies.iter() {
            empty_rows.remove(&galaxy.x);
            empty_cols.remove(&galaxy.y);
        }

        for (idx, galaxy) in self.galaxies.clone().into_iter().enumerate() {
            let row_shift: u32 = empty_rows.iter().filter(|&row| *row < galaxy.x).count() as u32;
            let col_shift: u32 = empty_cols.iter().filter(|&col| *col < galaxy.y).count() as u32;

            self.galaxies[idx] = Galaxy::new(galaxy.x + row_shift, galaxy.y + col_shift);
        }
    }

    fn total_distance(&self) -> u32 {
        let mut distance: u32 = 0;
        for (idx1, g1) in self.galaxies.iter().enumerate() {
            for idx2 in (idx1 + 1)..self.galaxies.len() {
                let g2 = self.galaxies[idx2];
                distance += manhattan_distance(*g1, g2);
            }
        }

        distance
    }
}

fn manhattan_distance(g1: Galaxy, g2: Galaxy) -> u32 {
    (g2.x as i32 - g1.x as i32).abs() as u32 + (g2.y as i32 - g1.y as i32).abs() as u32
}

fn part1(input: &str) -> u32 {
    let lines = input.lines();

    let mut galaxies: Vec<Galaxy> = Vec::new();
    for (row, line) in lines.enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch == '#' {
                galaxies.push(Galaxy::new(row as u32, col as u32));
            }
        }
    }

    let mut galaxy_cluster: GalaxyMap = GalaxyMap::new(galaxies.clone());

    galaxy_cluster.add_offset();

    galaxy_cluster.total_distance()
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = part1("...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....");
        assert_eq!(result, 374);
    }
}
