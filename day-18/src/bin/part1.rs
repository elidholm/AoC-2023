fn main() {
    let input = include_str!("./input18.txt");
    let output = part1(input);
    dbg!(output);
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Position(i32, i32);

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Direction(i32, i32);

fn calc_filled(vertices: &Vec<Position>) -> i32 {
    let mut area: i32 = 0;
    let mut perimeter: i32 = 0;
    for i in 0..(vertices.len() - 1) {
        let mut window = vertices[i..=(i + 1)].iter();
        let first: &Position = window.next().unwrap();
        let second: &Position = window.next().unwrap();
        area += (first.0 * second.1) - (first.1 * second.0);
        if first.0 == second.0 {
            perimeter += i32::abs(first.1 - second.1);
        } else if first.1 == second.1 {
            perimeter += i32::abs(first.0 - second.0);
        }
    }
    ((i32::abs(area) / 2) - (perimeter / 2) + 1) + perimeter
}

fn part1(input: &str) -> i32 {
    let start: Position = Position(0, 0);
    let mut vertices: Vec<Position> = Vec::new();
    vertices.push(start);
    let mut pos: Position = start;
    for line in input.lines() {
        let dir_part: &str = line.split(' ').next().unwrap();
        let steps: i32 = line.split(' ').nth(1).unwrap().parse().unwrap();
        let dir = match dir_part {
            "U" => Direction(0, 1),
            "D" => Direction(0, -1),
            "L" => Direction(-1, 0),
            "R" => Direction(1, 0),
            _ => panic!("unknown direction!"),
        };
        pos.0 += steps * dir.0;
        pos.1 += steps * dir.1;
        vertices.push(pos);
    }

    calc_filled(&vertices)
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = part1("R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)");
        assert_eq!(result, 62);
    }
}
