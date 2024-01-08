use std::collections::HashMap;

fn main() {
    let input = include_str!("./input15.txt");
    let output = part2(input);
    dbg!(output);
}

fn parse_lens_info(input: &str) -> (u32, char, Option<u32>) {
    let mut box_nr: u32 = 0;
    let mut operation: char = ' ';
    let mut focal_length: Option<u32> = None;

    for c in input.chars().filter(|&c| c != '\n') {
        if ['-', '='].contains(&c) {
            operation = c;
            continue;
        }
        if c.is_digit(10) {
            focal_length = Some(c.to_digit(10).unwrap());
            break;
        }
        box_nr += c as u32;
        box_nr *= 17;
        box_nr = box_nr % 256;
    }

    (box_nr, operation, focal_length)
}

fn perform_operation<'a>(step: &'a str, box_hash: &mut HashMap<u32, Vec<(&'a str, u32)>>) {
    let (box_nr, operation, focal_length) = parse_lens_info(step);
    let label: &str = step.split(['-', '=']).nth(0).unwrap();

    match operation {
        '-' => {
            if box_hash.contains_key(&box_nr) {
                let mut v: Vec<(&str, u32)> = box_hash.get(&box_nr).unwrap().to_vec();
                v.retain(|&l| l.0 != label);
                box_hash.insert(box_nr, v);
            }
        },
        '=' => {
            if box_hash.contains_key(&box_nr) {
                let mut v: Vec<(&str, u32)> = box_hash.get(&box_nr).unwrap().to_vec();
                if let Some(pos) = v.iter().position(|&l| l.0 == label) {
                    v[pos].1 = focal_length.unwrap_or(0);
                } else {
                    v.push((label, focal_length.unwrap_or(0)));
                }
                box_hash.insert(box_nr, v);
            } else {
                box_hash.insert(box_nr, vec![(label, focal_length.unwrap_or(0))]);
            }
        },
        _ => panic!("Unknown operation: {}", operation),
    }
}

fn focusing_power(box_hash: &HashMap<u32, Vec<(&str, u32)>>) -> u32 {
    let mut power: u32 = 0;
    for (box_number, v) in box_hash {
        for (idx, (_, f)) in v.into_iter().enumerate() {
            power += (box_number + 1) * (idx as u32 + 1) * f;
        }
    }
    power
}

fn part2(input: &str) -> u32 {
    let steps: Vec<&str> = input.split(",").collect();
    let mut box_hash: HashMap<u32, Vec<(&str, u32)>> = HashMap::new();


    for step in steps {
        perform_operation(step, &mut box_hash);
    }


    focusing_power(&box_hash)
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = part2("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7");
        assert_eq!(result, 145);
    }
}
