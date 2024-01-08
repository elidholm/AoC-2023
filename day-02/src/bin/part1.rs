fn main() {
    let input = include_str!("./input2.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> u32 {
    let lines = input.lines();
    let mut secret: u32 = 0;

    for line in lines {
        let mut game = line.split(":");
        let game_id: u32 = game.next().unwrap().split_whitespace().nth(1).unwrap().parse().unwrap();


        let mut possible_game = true;

        for deez in game.nth(0).unwrap().split([',', ';']) {
            let mut nuts = deez.split_whitespace();
            let number: u32 = nuts.nth(0).unwrap().parse().unwrap();
            let color: &str = nuts.nth(0).unwrap();
            if (color == "red" && number > 12) || (color == "green" && number > 13) || (color == "blue" && number > 14) {
                possible_game = false;
                break;
            }
        }

        if possible_game {
            secret += game_id;
        }
    }
    secret
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = part1("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green");
        assert_eq!(result, 8);
    }
}

