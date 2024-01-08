use std::collections::{ HashMap, VecDeque };

fn main() {
    let input = include_str!("./input19.txt");
    let output = part2(input);
    dbg!(output);
}

#[derive(Clone, Debug)]
struct Workflow {
    pub name: String,
    pub rules: Vec<Rule>,
    pub default: String,
}

impl Workflow {
    fn new(name: String, rules: Vec<Rule>, default: String) -> Self {
        Workflow { name, rules, default }
    }

    fn from_string(input: String) -> Self {
        let mut rules: Vec<Rule> = Vec::new();
        let mut default: String = String::new();
        let mut wf = input.split(['{','}']);
        let name = wf.next().unwrap().to_string();
        let rules_raw = wf.next().unwrap().split(',');
        for rule in rules_raw {
            if rule.contains(':') {
                rules.push(Rule::from_str(rule));
            } else {
                default = rule.to_string();
            }
        }

        Self::new(name, rules, default)
    }

    fn apply_rules(&self, mut range: Range) -> Vec<(Range, String)> {
        let mut split_ranges: Vec<(Range, String)> = Vec::new();

        for rule in &self.rules {
            if let Some((value, op)) = range.apply_rule(rule) {
                if let Some((left, right)) = range.split(op, value) {
                    match rule.operation {
                        '<' => {
                            split_ranges.push((left, rule.destination.clone()));
                            range = right;
                        }
                        '>' => {
                            split_ranges.push((right, rule.destination.clone()));
                            range = left;
                        }
                        _ => panic!("Invalid operation"),
                    }
                }
            }
        }
        split_ranges.push((range.clone(), self.default.clone()));
        split_ranges
    }
}

#[derive(Clone, Debug)]
struct Rule {
    pub category: char,
    pub operation: char,
    pub value: usize,
    pub destination: String,
}

impl Rule {
    fn new(category: char, operation: char, value: usize, destination: String) -> Self {
        Rule { category, operation, value, destination }
    }

    fn from_str(input: &str) -> Self {
        let mut parts = input.split(':');
        let mut deez = parts.next().unwrap().chars();
        let category = deez.next().unwrap();
        let operation = deez.next().unwrap();
        let value = deez.collect::<String>().parse::<usize>().unwrap();
        let destination = parts.next().unwrap();

        Self::new(category, operation, value, destination.to_string())
    }
}

#[derive(Clone, Debug)]
struct Range {
    pub x: (usize, usize),
    pub m: (usize, usize),
    pub a: (usize, usize),
    pub s: (usize, usize),
}

impl Range {
    fn new(x: (usize, usize), m: (usize, usize), a: (usize, usize), s: (usize, usize)) -> Self {
        Range { x, m, a, s }
    }

    fn get_value(&self) -> usize {
        (self.x.1 - self.x.0 + 1)  * (self.m.1 - self.m.0 + 1) * (self.a.1 - self.a.0 + 1) * (self.s.1 - self.s.0 + 1)
    }

    fn split(&self, category: char, value: usize) -> Option<(Self, Self)> {
        match category {
            'x' => {
                if self.x.0 < value && value <= self.x.1 {
                    let left = Range::new((self.x.0.clone(), value - 1), self.m.clone(), self.a.clone(), self.s.clone());
                    let right = Range::new((value, self.x.1.clone()), self.m.clone(), self.a.clone(), self.s.clone());
                    return Some((left, right));
                }
                return None;
            }
            'm' => {
                if self.m.0 < value && value <= self.m.1 {
                    let left = Range::new(self.x.clone(), (self.m.0.clone(), value - 1), self.a.clone(), self.s.clone());
                    let right = Range::new(self.x.clone(), (value, self.m.1.clone()), self.a.clone(), self.s.clone());
                    return Some((left, right));
                }
                return None;
            }
            'a' => {
                if self.a.0 < value && value <= self.a.1 {
                    let left = Range::new(self.x.clone(), self.m.clone(), (self.a.0.clone(), value - 1), self.s.clone());
                    let right = Range::new(self.x.clone(), self.m.clone(), (value, self.a.1.clone()), self.s.clone());
                    return Some((left, right));
                }
                return None;
            }
            's' => {
                if self.s.0 < value && value <= self.s.1 {
                    let left = Range::new(self.x.clone(), self.m.clone(), self.a.clone(), (self.s.0.clone(), value - 1));
                    let right = Range::new(self.x.clone(), self.m.clone(), self.a.clone(), (value, self.s.1.clone()));
                    return Some((left, right));
                }
                return None;
            }
            _ => panic!("Invalid category"),
        }
    }

    fn apply_rule(&self, rule: &Rule) -> Option<(usize, char)> {
        match rule.category {
            'x' => {
                match rule.operation {
                    '<' => {
                        if self.x.0.clone() < rule.value && rule.value <= self.x.1.clone() {
                            return Some((rule.value, 'x'));
                        }
                        return None;
                    }
                    '>' => {
                        if self.x.0.clone() <= rule.value && rule.value < self.x.1.clone() {
                            return Some((rule.value + 1, 'x'));
                        }
                        return None;
                    }
                    _ => panic!("Invalid operation"),
                }
            }
            'm' => {
                match rule.operation {
                    '<' => {
                         if self.m.0.clone() < rule.value && rule.value <= self.m.1.clone() {
                            return Some((rule.value, 'm'));
                         }
                        return None;
                    }
                    '>' => {
                        if self.m.0.clone() <= rule.value && rule.value < self.m.1.clone() {
                            return Some((rule.value + 1, 'm'));
                        }
                        return None;
                    }
                    _ => panic!("Invalid operation"),
                }
            }
            'a' => {
                match rule.operation {
                    '<' => {
                        if self.a.0.clone() < rule.value && rule.value <= self.a.1.clone() {
                            return Some((rule.value, 'a'));
                        }
                        return None;
                    }
                    '>' => {
                        if self.a.0.clone() <= rule.value && rule.value < self.a.1.clone() {
                            return Some((rule.value + 1, 'a'));
                        }
                        return None;
                    }
                    _ => panic!("Invalid operation"),
                }
            }
            's' => {
                match rule.operation {
                    '<' => {
                         if self.s.0.clone() < rule.value && rule.value <= self.s.1.clone() {
                            return Some((rule.value, 's'));
                         }
                        return None;
                    }
                    '>' => {
                         if self.s.0.clone() <= rule.value && rule.value < self.s.1.clone() {
                            return Some((rule.value + 1, 's'));
                         }
                        return None;
                    }
                    _ => panic!("Invalid operation"),
                }
            }
            _ => panic!("Invalid category"),
        }
    }
}



fn part2(input: &str) -> usize {
    let mut secret: usize = 0;
    let max: usize = 4000;
    let min: usize = 1;

    let mut ranges: VecDeque<(Range, String)> = VecDeque::new();
    ranges.push_back((Range::new((min, max), (min, max), (min, max), (min, max)), String::from("in")));

    let mut workflows: HashMap<String, Workflow> = HashMap::new();
    let mut split_input = input.split("\n\n");
    let workflows_raw = split_input.next().unwrap().lines();
    for wf in workflows_raw {
        let workflow: Workflow = Workflow::from_string(String::from(wf));
        workflows.insert(workflow.name.clone(), workflow);
    }

    while ranges.len() > 0 {
        let (range, next) = ranges.pop_front().unwrap();
        if next == "A" {
            secret += range.get_value();
            continue;
        } else if next == "R" {
            continue;
        }
        if let Some(wf) = workflows.get(&next) {
            let split_ranges: Vec<(Range, String)> = wf.apply_rules(range);
            ranges.extend(split_ranges);
        }
    }

    secret
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = part2("px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}");
        assert_eq!(result, 167409079868000);
    }
}
