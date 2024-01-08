//////////////////////////////////////////////////////////////////////////////
//                                                                          //
// Make no mistake, this is a TERRIBLE solution.                            //
// It's slow, it's ugly, but it WILL yield the correct solution...          //
// ...after ~30 minutes of running...                                       //
//                                                                          //
// I'm not proud of this, but I'm also not going to spend more time on it.  //
//                                                                          //
//////////////////////////////////////////////////////////////////////////////
use std::collections::HashSet;

fn main() {
    let input = include_str!("./input23.txt");
    let output = part2(input);
    dbg!(output);
}

type Map = Vec<Vec<char>>;
type Decision = Option<((usize, usize), (usize, usize))>;

#[derive(Debug, Clone)]
struct Hike {
    pub visited: HashSet<(usize, usize)>,
    pub current: (usize, usize),
    pub last_decision: Decision,
}

impl Hike {
    fn new(visited: HashSet<(usize, usize)>, current: (usize, usize), last_decision: Decision) -> Self {
        Self { visited, current, last_decision }
    }
}

fn parse_input(input: &str) -> (Map, (usize, usize), (usize, usize)) {
    let map: Map = input.lines().map(|line| line.chars().collect()).collect();
    let start: (usize, usize) = _find_start(&map);
    let end: (usize, usize) = _find_end(&map);

    (map, start, end)
}

fn _find_start(map: &Map) -> (usize, usize) {
    return (0, map[0].iter().position(|x| *x == '.').unwrap());
}

fn _find_end(map: &Map) -> (usize, usize) {
    let n_rows: usize = map.len();
    return (n_rows - 1, map[n_rows - 1].iter().position(|x| *x == '.').unwrap());
}

fn part2(input: &str) -> usize {
    let (map, start, end): (Map, (usize, usize), (usize, usize))  = parse_input(input);

    let mut hikes: Vec<Hike> = Vec::new();
    let mut deez: HashSet<(usize, usize)> = HashSet::new();
    deez.insert(start);
    let mut longest_hike: Hike = Hike::new(deez, start, None);
    let mut dead_ends: HashSet<Decision> = HashSet::new();

    hikes.push(longest_hike.clone());

    let mut n_hikes: usize = 0;

    while hikes.len() > 0 {
        let hike: Hike = hikes.pop().unwrap();
        let (y, x): (usize, usize) = hike.current;

        if hike.current == end {
            n_hikes += 1;
            if hike.visited.len() > longest_hike.visited.len() {
                longest_hike = hike;
                println!("Finished hike nr: {},\tLongest: {},\tActive hikes: {}", n_hikes, longest_hike.visited.len() - 1, hikes.len());
            }
            continue;
        }

        let x_temp: isize = x as isize;
        let y_temp: isize = y as isize;

        let possible_steps = vec![(y_temp, x_temp + 1), (y_temp + 1, x_temp), (y_temp, x_temp - 1), (y_temp - 1, x_temp)]
                                    .into_iter()
                                    .filter(|(y, x)| *y >= 0 &&
                                                        *x >= 0 &&
                                                        map[*y as usize][*x as usize] != '#' &&
                                                        !dead_ends.contains(&Some((hike.current, (*y as usize, *x as usize)))))
                                    .map(|(y, x)| (y as usize, x as usize))
                                    .collect::<Vec<(usize, usize)>>();

        match possible_steps.len() {
            0 => {
                dead_ends.insert(hike.last_decision.clone());
            }
            1 => {
                let next = possible_steps[0];
                if !hike.visited.contains(&next) {
                    let mut deez: HashSet<(usize, usize)> = hike.visited.clone();
                    deez.insert(next);
                    hikes.push(Hike::new(deez, next, hike.last_decision.clone()));
                }
            }
            _ => {
                for next in possible_steps {
                    if !hike.visited.contains(&next) {
                        let mut deez: HashSet<(usize, usize)> = hike.visited.clone();
                        deez.insert(next);
                        hikes.push(Hike::new(deez, next, Some((hike.current, next))));
                    }
                }
            }
        }
    }

    longest_hike.visited.len() - 1
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2("#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#");
        assert_eq!(result, 154);
    }
}
