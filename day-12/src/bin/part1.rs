use std::collections::HashMap;

type Cache = HashMap<(Vec<char>, Vec<usize>), usize>;

fn main() {
    let input = include_str!("./input12.txt");
    let output = part1(input);
    dbg!(output);
}

fn get_number_of_sols(springs: &Vec<char>, sizes: &Vec<usize>, cache: &mut Cache) -> usize {
    // check cache first
    if let Some(&result) = cache.get(&(springs.clone(), sizes.clone())) {
        return result;
    }

    if sizes.is_empty() {
        return !springs.contains(&'#') as usize;
    }

    let min_remaining: usize = sizes.iter().sum::<usize>() + sizes.len() - 1; // 1 for each space between groups

    if springs.len() < min_remaining {
        return 0;
    }

    let result: usize = match springs[0] {
        '.' => get_number_of_sols(&Vec::from(&springs[1..]), sizes, cache),
        '#' => remove_group(springs, sizes, cache),
        '?' => get_number_of_sols(&Vec::from(&springs[1..]), sizes, cache) + remove_group(springs, sizes, cache),
        _ => panic!("Invalid character in input"),
    };

    cache.insert((springs.clone(), sizes.clone()), result);
    result
}

fn remove_group(springs: &Vec<char>, sizes: &Vec<usize>, cache: &mut Cache) -> usize {
    if springs.len() < sizes[0] || springs[..sizes[0]].contains(&'.') {
        return 0;
    }
    if springs.len() == sizes[0] {
        return (sizes.len() == 1) as usize;
    }
    if springs[sizes[0]] == '#' {
        return 0;
    }

    get_number_of_sols(&Vec::from(&springs[sizes[0] + 1..]), &Vec::from(&sizes[1..]), cache)
}

fn part1(input: &str) -> usize {
    let lines = input.lines();
    let mut secret: usize = 0;

    for line in lines {
        let (springs_raw, groups_raw) = line.split_once(' ').unwrap();
        let springs: Vec<char> = springs_raw.chars().collect();
        let groups: Vec<usize> = groups_raw.split(",").map(|x| x.parse::<usize>().unwrap()).collect();

        let mut cache: Cache = HashMap::new();

        secret += get_number_of_sols(&springs, &groups, &mut cache);
    }

    secret
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = part1("???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1");
        assert_eq!(result, 21);
    }
}
