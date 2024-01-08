fn main() {
    let input = include_str!("./input15.txt");
    let output = part1(input);
    dbg!(output);
}

fn hash_algo(input: &str) -> u32 {
    let mut hash: u32 = 0;

    for c in input.chars() {
        if c == '\n' {
            continue;
        }
        hash += c as u32;
        hash *= 17;
        hash = hash % 256;
    }

    hash
}

fn part1(input: &str) -> u32 {
    let mut secret: u32 = 0;

    let steps: Vec<&str> = input.split(",").collect();
    
    for step in steps {
        let hash: u32 = hash_algo(step);
        secret += hash;
    }

    secret
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = part1("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7");
        assert_eq!(result, 1320);
    }
}
