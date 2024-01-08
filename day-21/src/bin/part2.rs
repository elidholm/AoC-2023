use std::collections::{ HashMap, HashSet };

fn main() {
    let input = include_str!("./input21.txt");
    let output = part2(input, 26501365);
    dbg!(output);
}

fn get_n_steps(side_length: usize, n_steps: usize, full_odd: usize, full_even: usize, points: usize, small_corners: usize, big_corners: usize) -> usize {
    let coeff = (n_steps - side_length/2) / side_length;

    let n_small_corners = coeff * small_corners;
    let n_big_corners = (coeff - 1) * big_corners;
    let n_points = points;
    let n_full_even = coeff.pow(2) * full_even;
    let n_full_odd = (coeff - 1).pow(2) * full_odd;

    n_small_corners + n_big_corners + n_points + n_full_even + n_full_odd
}

fn bfs(garden: &Vec<Vec<char>>, starting_points: Vec<(usize, usize)>, n_steps: usize, starting_step: usize) -> usize {
    let mut result = 0;
    let parity = n_steps % 2;

    let n_rows: usize = garden.len();
    let n_cols: usize = garden[0].len();

    for start in starting_points {
        let mut positions: HashMap<(usize, usize), usize> = HashMap::new();
        let mut frontier: HashSet<(usize, usize)> = HashSet::new();
        frontier.insert(start);

        for step in starting_step..=n_steps {
            let mut new_frontier: HashSet<(usize, usize)> = HashSet::new();
            for (i, j) in frontier.clone() {
                if i > 0 && garden[i-1][j] != '#'  && !positions.contains_key(&(i - 1, j)) {
                    new_frontier.insert((i - 1, j));
                }
                if  i < n_rows -1 && garden[i+1][j] != '#'  && !positions.contains_key(&(i + 1, j)) {
                    new_frontier.insert((i + 1, j));
                }
                if  j > 0 && garden[i][j-1] != '#'  && !positions.contains_key(&(i, j - 1)) {
                    new_frontier.insert((i, j - 1));
                }
                if j < n_cols - 1 && garden[i][j+1] != '#'  && !positions.contains_key(&(i, j + 1)) {
                    new_frontier.insert((i, j + 1));
                }

                positions.insert((i,j), step);

            }

            frontier = new_frontier;
        }

        result += positions.values().filter(|f| **f % 2 == parity).count();
        if frontier.len() == 0 {
            break;
        }
    }

    result
}

fn find_start(garden: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    let n_rows: usize = garden.len();
    let n_cols: usize = garden[0].len();

    for i in 0..n_rows {
        for j in 0..n_cols {
            if garden[i][j] == 'S' {
                return Some((i, j));
            }
        }
    }
    None
}

fn part2(input: &str, n_steps: usize) -> usize {
    let garden: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let n_rows: usize = garden.len();
    let n_cols: usize = garden[0].len();

    if let Some(start) = find_start(&garden) {
        let full_even = bfs(&garden, vec![start], n_steps, 1);
        let full_odd = bfs(&garden, vec![start], n_steps, 0);

        let sides: Vec<(usize, usize)> = vec![(n_rows/2, 0), (0, n_cols/2), (n_rows/2, n_cols-1), (n_rows-1, n_cols/2)];
        let points = bfs(&garden, sides, n_steps, n_steps-130);

        let corners: Vec<(usize, usize)> = vec![(0, 0), (0, n_cols-1), (n_rows-1, 0), (n_rows-1, n_cols-1)];
        let small_corners = bfs(&garden, corners.clone(), n_steps, n_steps-64);
        let big_corners = bfs(&garden, corners.clone(), n_steps, n_steps-195);

        get_n_steps(n_rows, n_steps, full_odd, full_even, points, small_corners, big_corners)
    } else {
        panic!("No start found");
    }
}
