fn main() {
    let input = include_str!("./input7.txt");
    let output = part2(input);
    dbg!(output);
}

#[derive(Debug, Clone)]
struct Hand {
    pub values: Vec<usize>,
    pub bet: usize,
}

impl Hand {
    fn new(hand_raw: &str) -> Self {
        let mut split_line = hand_raw.split_whitespace();
        let hand: &str = split_line.next().unwrap();
        let bet: usize = split_line.next().unwrap().parse::<usize>().unwrap();

        let mut values: Vec<usize> = Vec::new();
        for c in hand.chars() {
            let value = match c {
                '2'..='9' => c.to_digit(10).unwrap(),
                'T' => 10,
                'J' => 1,
                'Q' => 12,
                'K' => 13,
                'A' => 14,
                _ => 0,
            };
            values.push(value as usize);
        }

        Self { values, bet }
    }

    fn hand_type(&self) -> usize {
        let n_jokers = self.values.iter().filter(|&n| *n == 1).count();

        let mut temp: Vec<usize> = Vec::new();
        for val in 2..=14 { // Skipping 1 (Joker)
            temp.push(self.values.iter().filter(|&n| *n == val).count());
        }
        temp.sort();
        temp.reverse();
        let (c1, c2) = (temp[0], temp[1]);

        match (c1 + n_jokers, c2) {
            (5, _) => return 6,
            (4, _) => return 5,
            (3, 2) => return 4,
            (3, _) => return 3,
            (2, 2) => return 2,
            (2, _) => return 1,
            _ => return 0,
        }
    }
}

struct Game {
    pub hands: Vec<Hand>,
}

impl Game {
    fn new(hands: Vec<Hand>) -> Self {
        Self { hands }
    }

    fn sort(&mut self) {
        self.hands.sort_by_key(|h| (h.hand_type(), h.values.clone()));
    }
}

fn part2(input: &str) -> usize {
    let lines = input.lines();
    let mut secret: usize = 0;

    let hands: Vec<Hand> = lines.map(|line| Hand::new(line)).collect();

    let mut game: Game = Game::new(hands);
    game.sort();

    for (rank, hand) in game.hands.iter().enumerate() {
        secret += hand.bet * (rank + 1);
    }

    secret
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = part2("32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483");
        assert_eq!(result, 5905);
    }
}
