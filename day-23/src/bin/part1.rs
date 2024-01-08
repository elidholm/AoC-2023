use std::collections::HashSet;

fn main() {
    let input = include_str!("./input23.txt");
    let output = part1(input);
    dbg!(output);
}

type Map = Vec<Vec<char>>;

#[derive(Debug, Clone)]
struct Hike {
    pub visited: HashSet<(usize, usize)>,
    pub current: (usize, usize),
}

impl Hike {
    fn new(visited: HashSet<(usize, usize)>, current: (usize, usize)) -> Self {
        Self { visited, current }
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

fn uphill(map: &Map, hike: &Hike) -> bool {
    let (y, x): (usize, usize) = hike.current;
    return (map[y][x] == '^' && hike.visited.contains(&(y - 1, x))) ||
        (map[y][x] == 'v' && hike.visited.contains(&(y + 1, x))) ||
        (map[y][x] == '>' && hike.visited.contains(&(y, x + 1))) ||
        (map[y][x] == '<' && hike.visited.contains(&(y, x - 1)));
}

fn part1(input: &str) -> usize {
    let (map, start, end): (Map, (usize, usize), (usize, usize))  = parse_input(input);

    let mut hikes: Vec<Hike> = Vec::new();
    let mut deez: HashSet<(usize, usize)> = HashSet::new();
    deez.insert(start);
    let mut longest_hike: Hike = Hike::new(deez, start);

    hikes.push(longest_hike.clone());

    while hikes.len() > 0 {
        let hike: Hike = hikes.pop().unwrap();
        let (y, x): (usize, usize) = hike.current;

        if hike.current == end || uphill(&map, &hike) {
            if hike.visited.len() > longest_hike.visited.len() {
                longest_hike = hike;
            }
            continue;
        }

        let x_temp: isize = x as isize;
        let y_temp: isize = y as isize;

        let possible_steps = vec![(y_temp, x_temp + 1), (y_temp + 1, x_temp), (y_temp, x_temp - 1), (y_temp - 1, x_temp)]
                                    .into_iter()
                                    .filter(|(y, x)| *y >= 0 &&
                                                        *x >= 0 &&
                                                        !hike.visited.contains(&(*y as usize,*x as usize)) &&
                                                        map[*y as usize][*x as usize] != '#')
                                    .map(|(y, x)| (y as usize, x as usize))
                                    .collect::<Vec<(usize, usize)>>();

        for next in possible_steps {
            let mut deez: HashSet<(usize, usize)> = hike.visited.clone();
            deez.insert(next);
            hikes.push(Hike::new(deez, next));
        }
    }

    longest_hike.visited.len() - 1
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1("#.#####################
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
        assert_eq!(result, 94);
    }
}
