fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> u32 {
    let lines = input.lines();
    let mut secret: u32 = 0;

    for line in lines {
        let mut first_digit: Option<char> = None;
        let mut last_digit: Option<char> = None;

        for c in line.chars() {
            if c.is_digit(10) {
                if first_digit.is_none() {
                    first_digit = Some(c);
                }
                last_digit = Some(c);
            }
        }
        if let (Some(first), Some(last)) = (first_digit, last_digit) {
            let first_num = first.to_digit(10).unwrap();
            let last_num = last.to_digit(10).unwrap();
            let combined_number = 10*first_num + last_num;

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
        let result = part1("1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet");
        assert_eq!(result, 142);
    }
}

