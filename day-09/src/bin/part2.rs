fn main() {
    let input = include_str!("./input9.txt");
    let output = part2(input);
    dbg!(output);
}

fn get_first_number(deez: &Vec<isize>) -> isize {
    if deez.iter().all(|&x| x == 0) {
        return 0;
    }

    let differences = deez.windows(2).map(|x| x[1] - x[0]).collect::<Vec<isize>>();

    deez.first().unwrap() - get_first_number(&differences)
}

fn part2(input: &str) -> isize {
    let lines = input.lines();
    let mut secret: isize = 0;

    for line in lines {
        let history: Vec<isize> = line.split_whitespace().map(|s| s.parse::<isize>().unwrap()).collect();
        secret += get_first_number(&history);
    }

    secret
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = part2(
"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45");
        assert_eq!(result, 2);
    }
}
