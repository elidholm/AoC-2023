use std::collections::HashMap;
use itertools::Itertools;

fn main() {
    let input = include_str!("./input8.txt");
    let output = part2(input);
    dbg!(output);
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn lcm(a: usize, b: usize) -> usize {
    (a * b) / gcd(a, b)
}

fn part2(input: &str) -> usize {
    let mut lines = input.lines();
    let mut secret: usize = 1;

    let mut rl_instructions = lines.next().unwrap().chars().collect::<Vec<char>>().into_iter().cycle();

    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();
    let mut starting_points: Vec<&str> = Vec::new();
    for line in lines.filter(|&x| !x.is_empty()) {
        let mut direction = line.split(" = ");
        let source = direction.next().unwrap();
        let destinations: (&str, &str) = direction.next().unwrap().trim_matches(|c| c == '(' || c == ')').split(", ").collect_tuple().unwrap();
        map.insert(source, destinations);

        if source.ends_with('A') {
            starting_points.push(source);
        }
    }


    for mut current in starting_points {
        let mut steps: usize = 0;

        while !current.ends_with('Z') {
            let next: &(&str, &str) = map.get(&current).unwrap();
            current = match rl_instructions.next().unwrap() {
                'L' => next.0,
                'R' => next.1,
                _ => panic!("Something went wrong!"),
            };
            steps += 1;
        }

        secret = lcm(secret, steps)
    }

    secret
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = part2("LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)");
        assert_eq!(result, 6);
    }
}
