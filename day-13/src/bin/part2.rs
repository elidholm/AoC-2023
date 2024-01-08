fn main() {
    let input = include_str!("./input13.txt");
    let output = part2(input);
    dbg!(output);
}

#[derive(Debug)]
struct Pattern {
    rows: Vec<String>,
    cols: Vec<String>,
}

impl Pattern {
    fn from(pattern: &str) -> Self {
        let rows: Vec<String> = pattern.lines().map(|l| l.to_string()).collect();

        let mut cols: Vec<String> = Vec::new();
        cols.resize_with(rows[0].len(), String::new);

        for row in rows.iter().map(|row| row.chars())  {
            for i in 0..rows[0].len() {
                cols[i].push(row.clone().nth(i).expect("Could not unpack character from row!"));
            }
        }

        Self { rows, cols }
    }
}

struct Notes {
    patterns: Vec<Pattern>,
}

impl Notes {
    fn from(input: &str) -> Self {
        let patterns: Vec<Pattern> = input.split("\n\n").map(|p| Pattern::from(p)).collect();
        Self { patterns }
    }
}

fn is_perfect_reflection(pattern: Vec<String>, coords: (usize, usize)) -> bool {
    let (mut i, mut j) = (coords.0, coords.1);

    let mut k = 0;

    loop {
        if pattern[i] != pattern[j] {
            let ne = pattern[i].chars().zip(pattern[j].chars()).filter(|(c1, c2)| c1 != c2).collect::<Vec<(char, char)>>().len();

            if ne > 1 {
                return false;
            } else {
                k += 1;
            }

            if k > 1 {
                return false;
            }
        }

        if i == 0 {
            break;
        } 

        if j == pattern.len() - 1 {
            break;
        }

        i -= 1;
        j += 1;
    }

    k == 1
}


fn find_reflection(pattern: Vec<String>) -> usize {
    let mut i = 0;
    let mut j = 1;

    while j < pattern.len() {
        let ne = pattern[i].chars().zip(pattern[j].chars()).filter(|(c1, c2)| c1 != c2).collect::<Vec<(char, char)>>().len();

        if ne < 2 {
            if is_perfect_reflection(pattern.clone(), (i, j)) {
                return j;
            }
        }

        i += 1;
        j += 1;
    }

    0
}


fn part2(input: &str) -> usize {
    let notes = Notes::from(input);

    notes
        .patterns
        .iter()
        .map(|p| {
            let vn = find_reflection(p.cols.clone());
            let hn = find_reflection(p.rows.clone());

            assert!(vn == 0 || hn == 0);
            assert!(vn != 0 || hn != 0);

            vn + (100 * hn)
        })
    .sum()
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = part2("#.##..##.
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
            assert_eq!(result, 400);
    }
}
