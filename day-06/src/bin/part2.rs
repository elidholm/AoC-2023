fn main() {
    let input = include_str!("./input6.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> usize {
    let mut lines = input.lines();
    let mut secret: usize = 0;

    let time_constraint: usize = lines
                                    .next()
                                    .unwrap()
                                    .split(":")
                                    .nth(1)
                                    .unwrap()
                                    .chars()
                                    .filter(|&c| c.is_digit(10))
                                    .fold(0, |acc, c| acc*10 + c.to_digit(10).unwrap() as usize);
    let record: usize = lines
                            .next()
                            .unwrap()
                            .split(":")
                            .nth(1)
                            .unwrap()
                            .chars()
                            .filter(|&c| c.is_digit(10))
                            .fold(0, |acc, c| acc*10 + c.to_digit(10).unwrap() as usize);

    let deez: Vec<usize> = (1..=time_constraint/2).collect();
    for t in deez {
        let d: usize = t * (time_constraint - t);
        if d > record {
            secret = time_constraint - 2 * t + 1;
            break;
        }
    }

    secret
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = part2("Time:      7  15   30
Distance:  9  40  200");
        assert_eq!(result, 71503);
    }
}

