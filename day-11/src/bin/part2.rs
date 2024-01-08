use std::collections::HashSet;

fn main() {
    let input = include_str!("./input11.txt");
    let output = part2(input, 1000000);
    dbg!(output);
}

#[derive(Clone, Copy, Debug)]
struct Galaxy {
    pub x: usize,
    pub y: usize,
}

impl Galaxy {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Debug)]
struct GalaxyMap {
    pub galaxies: Vec<Galaxy>,
}

impl GalaxyMap {
    fn new(galaxies: Vec<Galaxy>) -> Self {
        Self { galaxies }
    }

    fn add_offset(&mut self, offset: usize) {
        let n_rows = self.galaxies.iter().map(|g| g.x).max().unwrap();
        let n_cols = self.galaxies.iter().map(|g| g.y).max().unwrap();

        let mut empty_rows: HashSet<usize> = (0..=n_rows).collect();
        let mut empty_cols: HashSet<usize> = (0..=n_cols).collect();
        for galaxy in self.galaxies.iter() {
            empty_rows.remove(&galaxy.x);
            empty_cols.remove(&galaxy.y);
        }

        for (idx, galaxy) in self.galaxies.clone().into_iter().enumerate() {
            let row_shift: usize = empty_rows.iter().filter(|&row| *row < galaxy.x).count() * (offset - 1);
            let col_shift: usize = empty_cols.iter().filter(|&col| *col < galaxy.y).count() * (offset - 1);

            self.galaxies[idx] = Galaxy::new(galaxy.x + row_shift, galaxy.y + col_shift);
        }
    }

    fn total_distance(&self) -> usize {
        let mut distance: usize = 0;
        for (idx1, g1) in self.galaxies.iter().enumerate() {
            for idx2 in (idx1 + 1)..self.galaxies.len() {
                let g2 = self.galaxies[idx2];
                distance += manhattan_distance(*g1, g2);
            }
        }

        distance
    }
}

fn manhattan_distance(g1: Galaxy, g2: Galaxy) -> usize {
    ((g2.x as isize - g1.x as isize).abs() + (g2.y as isize - g1.y as isize).abs()) as usize
}

fn part2(input: &str, offset: usize) -> usize {
    let lines = input.lines();

    let mut galaxies: Vec<Galaxy> = Vec::new();
    for (row, line) in lines.enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch == '#' {
                galaxies.push(Galaxy::new(row, col));
            }
        }
    }

    let mut galaxy_cluster: GalaxyMap = GalaxyMap::new(galaxies.clone());

    galaxy_cluster.add_offset(offset);

    galaxy_cluster.total_distance()
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = part2("...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....", 10);
        assert_eq!(result, 1030);
    }

    #[test]
    fn it_still_works() {
        let result = part2("...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....", 100);
        assert_eq!(result, 8410);
    }
}
