fn main() {
    let input = include_str!("./input6.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> u32 {
    let mut lines = input.lines();

    let mut secret: u32 = 1;

    let time_constraints: Vec<u32> = lines
                                .next()
                                .unwrap()
                                .split(":")
                                .nth(1)
                                .unwrap()
                                .split_whitespace()
                                .filter(|&x| !x.is_empty())
                                .map(|x| x.parse::<u32>().unwrap())
                                .collect();
    let records: Vec<u32> = lines
                                .next()
                                .unwrap()
                                .split(":")
                                .nth(1)
                                .unwrap()
                                .split_whitespace()
                                .filter(|&x| !x.is_empty())
                                .map(|x| x.parse::<u32>().unwrap())
                                .collect();

    let test = time_constraints.iter().zip(records.iter());
    for (time, record) in test {
        let deez: Vec<u32> = (1..=*time).collect();
        let nuts: Vec<u32> = deez.iter().filter(|&t| (t * (time - t)) > *record).cloned().collect();
        secret *= nuts.len() as u32;
    }

    secret
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = part1("Time:      7  15   30
Distance:  9  40  200");
        assert_eq!(result, 288);
    }
}

