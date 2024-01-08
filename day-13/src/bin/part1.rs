use std::collections::HashMap;

type Cache = HashMap<Vec<char>, Vec<u32>>;

fn main() {
    let input = include_str!("./input13.txt");
    let output = part1(input);
    dbg!(output);
}

fn cache_rows(group: &str) -> Cache {
    let mut row_cache: Cache = HashMap::new();
    for (row, line) in group.lines().enumerate() {
        let line = line.chars().collect::<Vec<char>>();
        if !row_cache.contains_key(&line) {
            row_cache.insert(line, vec![(row + 1) as u32; 1]);
        } else {
            let mut v = row_cache.get(&line).unwrap().to_vec();
            v.push((row + 1) as u32);
            row_cache.insert(line, v);
        }
    }
    row_cache
}

fn cache_cols(group: &str) -> Cache {
    let mut col_cache: Cache = HashMap::new();
    let n_cols: usize = group.lines().next().unwrap().len();

    let deez: Vec<Vec<char>> = group.lines().map(|l| l.chars().collect::<Vec<char>>()).collect();

    let transposed: Vec<Vec<char>> = (0..n_cols).map(|i| deez.iter().map(|row| row[i]).collect()).collect();

    for (col, line) in transposed.into_iter().enumerate() {
        if !col_cache.contains_key(&line) {
            col_cache.insert(line, vec![(col + 1) as u32; 1]);
        } else {
            let mut v = col_cache.get(&line).unwrap().to_vec();
            v.push((col + 1) as u32);
            col_cache.insert(line, v);
        }
    }
    col_cache
}

fn check_for_horisontal_mirror(row_cache: Cache, n_rows: u32) -> Option<u32> {
    for val in row_cache.values() {
        if val.len() > 1 {
            for i in 0..val.len() - 1 {
                if val[i + 1] - val[i] == 1 as u32 {
                    let mut top_line: u32 = val[i];
                    let mut bottom_line: u32 = val[i + 1];
                    let refl_line: u32 = top_line;

                    while row_cache.values().any(|v| v.iter().any(|&l| l == (top_line - 1)) && v.iter().any(|&l| l == (bottom_line + 1)))  {
                        top_line -= 1;
                        bottom_line += 1;
                    }
                    if top_line == 1 || bottom_line == n_rows {
                        return Some(100 * refl_line);
                    }
                }
            }
        }
    }
    None
}

fn check_for_vertical_mirror(col_cache: Cache, n_cols: u32) -> Option<u32> {
    for val in col_cache.values() {
        if val.len() > 1 {
            for i in 0..val.len() - 1 {
                if val[i + 1] - val[i] == 1 as u32 {
                    let mut left_col: u32 = val[i];
                    let mut right_col: u32 = val[i + 1];
                    let refl_col: u32 = left_col;

                    while col_cache.values().any(|v| v.iter().any(|&l| l == (left_col - 1)) && v.iter().any(|&l| l == (right_col + 1)))  {
                        left_col -= 1;
                        right_col += 1;
                    }
                    if left_col == 1 || right_col == n_cols {
                        return Some(refl_col);
                    }
                }
            }
        }
    }
    None
}

fn part1(input: &str) -> u32 {
    let mut secret: u32 = 0;

    let deez: Vec<&str> = input.split("\n\n").collect();

    for group in deez {
        let n_rows: u32 = group.lines().count().try_into().unwrap();
        let n_cols: u32 = group.lines().next().unwrap().len().try_into().unwrap();
        let row_cache: Cache = cache_rows(group);
        if let Some(result) = check_for_horisontal_mirror(row_cache, n_rows) {
            secret += result;
            continue;
        }
        let col_cache: Cache = cache_cols(group);
        if let Some(result) = check_for_vertical_mirror(col_cache, n_cols) {
            secret += result;
        }
    }

    secret
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = part1("#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#");
        assert_eq!(result, 405);
    }
}
