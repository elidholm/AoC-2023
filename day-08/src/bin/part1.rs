use std::collections::HashMap;
use itertools::Itertools;

fn main() {
    let input = include_str!("./input8.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> usize {
    let mut lines = input.lines();
    let start: &str = "AAA";
    let end: &str = "ZZZ";
    let mut secret: usize = 0;

    let mut rl_instructions = lines.next().unwrap().chars().collect::<Vec<char>>().into_iter().cycle();

    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();
    for line in lines.filter(|&x| !x.is_empty()) {
        let mut direction = line.split(" = ");
        let source: &str = direction.next().unwrap();
        let destinations: (&str, &str) = direction.next().unwrap().trim_matches(|c| c == '(' || c == ')').split(", ").collect_tuple().unwrap();

        map.insert(source, destinations);
    }

    let mut current = start;
    while current != end {
        let next: &(&str, &str) = map.get(&current).unwrap();
        current = match rl_instructions.next().unwrap() {
            'L' => next.0,
            'R' => next.1,
            _ => panic!("Something went wrong!"),
        };
        secret += 1;
    }

    secret
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = part1("RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)");
        assert_eq!(result, 2);
    }

    #[test]
    fn it_still_works() {
        let result = part1("LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)");
        assert_eq!(result, 6);
    }
}
