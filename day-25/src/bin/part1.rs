use std::collections::{ HashSet, HashMap, VecDeque };

fn main() {
    let input = include_str!("./input25.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> usize {
    let lines = input.lines();
    let mut edges: HashMap<&str, HashSet<&str>> = HashMap::new();

    for line in lines {
        let components: Vec<&str> = line.split_ascii_whitespace().collect();

        let key: &str = &components[0][..3];
        let parent: &mut HashSet<&str> = edges.entry(key).or_insert(HashSet::new());

        for &child in &components[1..] {
            parent.insert(&child);
        }

        for &child in &components[1..] {
            let entry: &mut HashSet<&str> = edges.entry(child).or_insert(HashSet::new());
            entry.insert(key);
        }
    }

    let mut freq: HashMap<(&str, &str), u32> = HashMap::new();

    for &start in edges.keys() {
        let mut todo: VecDeque<&str> = VecDeque::new();
        todo.push_back(start);

        let mut seen: HashSet<&str> = HashSet::new();
        seen.insert(start);

        while let Some(pos) = todo.pop_front() {
            for &next in &edges[pos] {
                if seen.insert(next) {
                    let key: (&str, &str) = if pos < next { (pos, next) } else { (next, pos) };

                    let entry: &mut u32 = freq.entry(key).or_insert(0);
                    *entry += 1;

                    todo.push_back(next);
                }
            }
        }
    }

    let mut order: Vec<(&(&str, &str), &u32)> = freq.iter().collect();
    order.sort_unstable_by_key(|edge| edge.1);
    order.reverse();

    let cut: Vec<(&str, &str)> = order.iter().take(3).map(|p| *p.0).collect();
    let start: &str = *edges.keys().next().unwrap();
    let mut size: usize = 1;

    let mut todo: VecDeque<&str> = VecDeque::new();
    todo.push_back(start);

    let mut seen:  HashSet<&str> = HashSet::new();
    seen.insert(start);

    while let Some(pos) = todo.pop_front() {
        for &next in &edges[pos] {
            let key: (&str, &str) = if pos < next { (pos, next) } else { (next, pos) };
            if cut.contains(&key) {
                continue;
            }

            if seen.insert(next) {
                size += 1;
                todo.push_back(next);
            }
        }
    }

    size * (edges.len() - size)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1("jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr");
        assert_eq!(result, 54);
    }
}
