use std::collections::HashMap;

fn main() {
    let input = include_str!("./input19.txt");
    let output = part1(input);
    dbg!(output);
}

#[derive(Clone, Debug)]
struct MachinePart {
    pub x: u32,
    pub m: u32,
    pub a: u32,
    pub s: u32,
}

impl MachinePart {
    fn new(x: u32, m: u32, a: u32, s: u32) -> Self {
        MachinePart { x, m, a, s }
    }

    fn from_string(input: String) -> Self {
        let parts = input.split([',', '{', '}']).filter(|x| !x.is_empty());
        let mut x = 0;
        let mut m = 0;
        let mut a = 0;
        let mut s = 0;
        for part in parts {
            let mut deez = part.split('=');
            let category = deez.next().unwrap();
            let value = deez.next().unwrap().parse::<u32>().unwrap();
            match category {
                "x" => x = value,
                "m" => m = value,
                "a" => a = value,
                "s" => s = value,
                _ => panic!("Invalid category"),
            }
        }
        Self::new(x, m, a, s)
    }

    fn get_value(&self) -> u32 {
        self.x + self.m + self.a + self.s
    }

    fn apply_rule(&self, rule: &Rule) -> Option<String> {
        match rule.category {
            'x' => {
                match rule.operation {
                    '<' => {
                        if self.x < rule.value {
                            Some(rule.destination.clone())
                        } else {
                            None
                        }
                    }
                    '>' => {
                        if self.x > rule.value {
                            Some(rule.destination.clone())
                        } else {
                            None
                        }
                    }
                    _ => panic!("Invalid operation"),
                }
            }
            'm' => {
                match rule.operation {
                    '<' =>  {
                        if self.m < rule.value {
                            Some(rule.destination.clone())
                        } else {
                            None
                        }
                    }
                    '>' => {
                        if self.m > rule.value {
                            Some(rule.destination.clone())
                        } else {
                            None
                        }
                    }
                    _ => panic!("Invalid operation"),
                }
            }
            'a' => {
                match rule.operation {
                    '<' => {
                        if self.a < rule.value {
                            Some(rule.destination.clone())
                        } else {
                            None
                        }
                    }
                    '>' => {
                        if self.a > rule.value {
                            Some(rule.destination.clone())
                        } else {
                            None
                        }
                    }
                    _ => panic!("Invalid operation"),
                }
            }
            's' => {
                match rule.operation {
                    '<' => {
                        if self.s < rule.value {
                            Some(rule.destination.clone())
                        } else {
                            None
                        }
                    }
                    '>' => {
                        if self.s > rule.value {
                            Some(rule.destination.clone())
                        } else {
                            None
                        }
                    }
                    _ => panic!("Invalid operation"),
                }
            }
            _ => panic!("Invalid category"),
        }
    }
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
        let name: String = String::from(wf.next().unwrap());
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

    fn apply_rules(&self, part: &MachinePart) -> String {
        for rule in &self.rules {
            if let Some(destination) = part.apply_rule(rule) {
                return destination;
            }
        }
        self.default.clone()
    }
}

#[derive(Clone, Debug)]
struct Rule {
    pub category: char,
    pub operation: char,
    pub value: u32,
    pub destination: String,
}

impl Rule {
    fn new(category: char, operation: char, value: u32, destination: String) -> Self {
        Rule { category, operation, value, destination }
    }

    fn from_str(input: &str) -> Self {
        let mut rule_raw = input.split(':');
        let mut condition = rule_raw.next().unwrap().chars();
        let category: char = condition.next().unwrap();
        let operation: char = condition.next().unwrap();
        let value: u32 = condition.collect::<String>().parse().unwrap();
        let destination: String = String::from(rule_raw.next().unwrap());

        Self::new(category, operation, value, destination)
    }
}



fn part1(input: &str) -> u32 {
    let mut secret: u32 = 0;
    let mut workflows: HashMap<String, Workflow> = HashMap::new();

    let mut split_input = input.split("\n\n");

    let workflows_raw = split_input.next().unwrap().lines();
    for wf in workflows_raw {
        let workflow: Workflow = Workflow::from_string(String::from(wf));
        workflows.insert(workflow.name.clone(), workflow);
    }

    let parts_raw = split_input.next().unwrap().lines();
    for p in parts_raw {
        let part: MachinePart = MachinePart::from_string(String::from(p));
        let mut next: &str = "in";
        let mut destination: String;

        while !["R", "A"].contains(&next) {
            let wf = workflows.get(next).unwrap();
            destination = wf.apply_rules(&part);
            next = &destination;
        }

        if next == "A" {
            secret += part.get_value();
        }
    }

    secret
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = part1("px{a<2006:qkq,m>2090:A,rfg}
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
        assert_eq!(result, 19114);
    }
}
