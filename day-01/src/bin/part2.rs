fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> u32 {
    let lines = input.lines();
    let mut secret = 0;

    for line in lines {
        let mut first_digit: Option<char> = None;
        let mut last_digit: Option<char> = None;

        for (i, c) in line.chars().enumerate() {
            if c.is_digit(10) {
                if first_digit.is_none() {
                    first_digit = Some(c);
                }
                last_digit = Some(c);
                continue;
            }

            match c {
                'o' => {
                    if line.len() - i > 2 && &line[i..i+3] == "one" {
                        if first_digit.is_none() {
                            first_digit = Some('1');
                        }
                        last_digit = Some('1');
                    }
                },
                't' => {
                    if line.len() - i > 2 && &line[i..i+3] == "two" {
                        if first_digit.is_none() {
                            first_digit = Some('2');
                        }
                        last_digit = Some('2');
                    } else if line.len() - i > 4  && &line[i..i+5] == "three" {
                        if first_digit.is_none() {
                            first_digit = Some('3');
                        }
                        last_digit = Some('3');
                    }
                },
                'f' => {
                    if line.len() - i > 3 {
                        match &line[i..i+4] {
                            "four" => {
                                if first_digit.is_none() {
                                    first_digit = Some('4');
                                }
                                last_digit = Some('4');
                            },
                            "five" => {
                                if first_digit.is_none() {
                                    first_digit = Some('5');
                                }
                                last_digit = Some('5');
                            },
                            _ => {},
                        }
                    }
                },
                's' => {
                    if line.len() - i > 2 && &line[i..i+3] == "six" {
                        if first_digit.is_none() {
                            first_digit = Some('6');
                        }
                        last_digit = Some('6');
                    } else if line.len() - i > 4 && &line[i..i+5] == "seven" {
                        if first_digit.is_none() {
                            first_digit = Some('7');
                        }
                        last_digit = Some('7');
                    }
                },
                'e' => {
                    if line.len() - i > 4 && &line[i..i+5] == "eight" {
                        if first_digit.is_none() {
                            first_digit = Some('8');
                        }
                        last_digit = Some('8');
                    }
                },
                'n' => {
                    if line.len() - i > 3 && &line[i..i+4] == "nine" {
                        if first_digit.is_none() {
                            first_digit = Some('9');
                        }
                        last_digit = Some('9');
                    }
                },
                _ => {},
            }
        }

        if let (Some(first), Some(last)) = (first_digit, last_digit) {
            let first_num: u32 = first.to_digit(10).unwrap();
            let last_num: u32 = last.to_digit(10).unwrap();
            let combined_number: u32 = 10*first_num + last_num;

            secret += combined_number;
        } else {
            println!("No number found in string: {}", line);
        }
    }

    secret
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = part2("two1nine
                           eightwothree
                           abcone2threexyz
                           xtwone3four
                           4nineeightseven2
                           zoneight234
                           7pqrstsixteen");
        assert_eq!(result, 281);
    }
}
