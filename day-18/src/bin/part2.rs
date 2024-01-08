fn main() {
    let input = include_str!("./input18.txt");
    let output = part2(input);
    dbg!(output);
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Position(isize, isize);

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Direction(isize, isize);

fn calc_filled(vertices: &Vec<Position>) -> isize {
    let mut area: isize = 0;
    let mut perimeter: isize = 0;
    for i in 0..(vertices.len() - 1) {
        let mut window = vertices[i..=(i + 1)].iter();
        let first: &Position = window.next().unwrap();
        let second: &Position = window.next().unwrap();
        area += (first.0 * second.1) - (first.1 * second.0);
        if first.0 == second.0 {
            perimeter += isize::abs(first.1 - second.1);
        } else if first.1 == second.1 {
            perimeter += isize::abs(first.0 - second.0);
        }
    }
    ((isize::abs(area) / 2) - (perimeter / 2) + 1) + perimeter
}

fn part2(input: &str) -> isize {
    let start: Position = Position(0, 0);
    let mut vertices: Vec<Position> = Vec::new();
    vertices.push(start);
    let mut pos: Position = start;
    for line in input.lines() {
        let encoded_instruction: &str = line.split_whitespace().last().unwrap();
        let steps: isize = isize::from_str_radix(&encoded_instruction[2..=6], 16).unwrap();
        let dir_part: &isize = &encoded_instruction[7..8].parse().unwrap();
        let dir: Direction = match dir_part {
            3 => Direction(0, 1),
            1 => Direction(0, -1),
            2 => Direction(-1, 0),
            0 => Direction(1, 0),
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
        let result = part2("R 6 (#70c710)
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
        assert_eq!(result, 952408144115);
    }
}
