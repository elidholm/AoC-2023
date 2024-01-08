fn main() {
    let input = include_str!("./input2.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> u32 {
    let lines = input.lines();
    let mut secret: u32 = 0;

    for line in lines {
        let mut highest_red = 0;
        let mut highest_green = 0;
        let mut highest_blue = 0;
        for deez in line.split(":").nth(1).unwrap().split([',', ';']) {
            let mut nuts = deez.split_whitespace();
            let number: u32 = nuts.next().unwrap().parse().unwrap();
            let color: &str = nuts.next().unwrap();
            if color == "red" && number > highest_red {
                highest_red = number;
            } else if color == "green" && number > highest_green {
                highest_green = number;
            } else if color == "blue" && number > highest_blue {
                highest_blue = number;
            }
        }
        let power: u32 = highest_red * highest_green * highest_blue;
        secret += power;
    }
    secret
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = part2("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green");
        assert_eq!(result, 2286);
    }
}

