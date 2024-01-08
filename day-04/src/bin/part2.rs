use std::cmp;

fn main() {
    let input = include_str!("./input4.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> u32 {
    let lines = input.lines();
    let n: usize = lines.clone().count();
    let mut n_cards = vec![1; n];

    for (idx, line) in lines.enumerate() {
        let mut numbers = line.split(":").nth(1).unwrap().split("|");
        let winning_numbers: Vec<u32> = numbers.next().unwrap().split_whitespace().map(|n| n.parse::<u32>().unwrap()).collect();
        let my_numbers: Vec<u32> = numbers.next().unwrap().split_whitespace().map(|n| n.parse::<u32>().unwrap()).collect();

        let nuts: usize = my_numbers.into_iter().filter(|&n| winning_numbers.contains(&n)).count();

        if nuts > 0 {
            let max: usize = cmp::min(n-1, idx + nuts);
            if idx < n-1 {
                for i in idx..max {
                    n_cards[i + 1] += n_cards[idx];
                }
            }
        }
    }

    n_cards.iter().sum()
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = part2("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11");
        assert_eq!(result, 30);
    }
}

