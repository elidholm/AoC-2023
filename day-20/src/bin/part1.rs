use std::collections::{ HashMap, VecDeque };

fn main() {
    let input = include_str!("./input20.txt");
    let output = part1(input);
    dbg!(output);
}

#[derive(Clone, Debug, PartialEq)]
enum PulseType {
    Low,
    High,
}

#[derive(Clone, Debug)]
enum ModuleState {
    On,
    Off,
}

#[derive(Clone, Debug)]
struct Pulse {
    pub pulse: PulseType,
    pub destination: String,
    pub source: String,
}

impl Pulse {
    fn new(pulse: PulseType, destination: String, source: String) -> Self {
        Pulse { pulse, destination, source }
    }
}

#[derive(Clone, Debug)]
struct Button {
    pulse: PulseType,
}

impl Button {
    fn new(pulse: PulseType) -> Self {
       Button { pulse }
    }

    fn press(&self) -> PulseType {
        self.pulse.clone()
    }
}

#[derive(Clone, Debug)]
struct FlipFlop {
    name: String,
    state: ModuleState,
    outputs: Vec<String>,
}

impl FlipFlop {
    fn new(name: String, outputs: Vec<String>) -> Self {
        FlipFlop { name, state: ModuleState::Off, outputs }
    }

    fn relay(&mut self, pulse: PulseType) -> Option<Vec<Pulse>> {
        let mut output_pulses: Vec<Pulse> = Vec::new();
        match pulse {
            PulseType::Low => {
                match self.state {
                    ModuleState::On => {
                        self.state = ModuleState::Off;
                        for output in &self.outputs {
                            output_pulses.push(Pulse::new(PulseType::Low, output.clone(), self.name.clone()));
                        }
                    }
                    ModuleState::Off => {
                        self.state = ModuleState::On;
                        for output in &self.outputs {
                            output_pulses.push(Pulse::new(PulseType::High, output.clone(), self.name.clone()));
                        }
                    }
                }
                Some(output_pulses)
            }
            PulseType::High => {
                None
            }
        }
    }
}

#[derive(Clone, Debug)]
struct Conjunction {
    name: String,
    inputs: HashMap<String, PulseType>,
    outputs: Vec<String>,
}

impl Conjunction {
    fn new(name: String, input_modules: Vec<String>, outputs: Vec<String>) -> Self {
        let mut inputs: HashMap<String, PulseType> = HashMap::new();
        for input_module in input_modules {
            inputs.insert(input_module, PulseType::Low);
        }
        Conjunction { name, inputs, outputs }
    }

    fn relay(&mut self, pulse: PulseType, source: String) -> Vec<Pulse> {
        self._update_state(source, pulse);
        let mut output_pulses: Vec<Pulse> = Vec::new();

        if self._all_high() {
            for output in &self.outputs {
                output_pulses.push(Pulse::new(PulseType::Low, output.clone(), self.name.clone()));
            }
            output_pulses
        } else {
            for output in &self.outputs {
                output_pulses.push(Pulse::new(PulseType::High, output.clone(), self.name.clone()));
            }
            output_pulses
        }
    }

    fn _update_state(&mut self, source: String, pulse: PulseType) {
        self.inputs.insert(source, pulse);
    }

    fn _all_high(&self) -> bool {
        for (_, pulse) in &self.inputs {
            if pulse == &PulseType::Low {
                return false;
            }
        }
        return true;
    }
}

#[derive(Clone, Debug)]
struct Broadcaster {
    outputs: Vec<String>,
}

impl Broadcaster {
    fn new(outputs: Vec<String>) -> Self {
        Broadcaster { outputs }
    }

    fn broadcast(&self, pulse: PulseType) -> Vec<Pulse> {
        let mut pulses: Vec<Pulse> = Vec::new();
        for dest in self.outputs.clone() {
            pulses.push(Pulse::new(pulse.clone(), dest.clone(), "broadcaster".to_string()));
        }
        pulses
    }
}

fn part1(input: &str) -> usize {
    let lines = input.lines();
    let button: Button = Button::new(PulseType::Low);
    let mut broadcaster: Broadcaster = Broadcaster::new(Vec::new());
    let mut flip_flops: HashMap<String, FlipFlop> = HashMap::new();
    let mut conjunctions: HashMap<String, Conjunction> = HashMap::new();
    let mut cons : Vec<(String, Vec<String>)> = Vec::new();
    let mut high_pulses: usize = 0;
    let mut low_pulses: usize = 0;

    for line in lines {
        let mut parts = line.split("->");

        let name: &str = parts.next().unwrap().trim();
        let outputs: Vec<String> = parts.next().unwrap().split(',').map(|s| s.trim().to_string()).collect();
        match name.chars().next().unwrap() {
            '%' => {
                flip_flops.insert(name[1..].to_string(), FlipFlop::new(String::from(&name[1..]), outputs));
            }
            '&' => {
                cons.push((name[1..].to_string(), outputs));
            }
            'b' => {
                broadcaster = Broadcaster::new(outputs);
            }
            _ => panic!("Unknown module type"),
        }
    }

    for con in &cons {
        let mut inputs: Vec<String> = Vec::new();
        for (name, ff) in &flip_flops {
            if ff.outputs.contains(&con.0) {
                inputs.push(name.clone());
            }
        }
        conjunctions.insert(con.0.clone(), Conjunction::new(con.0.clone(), inputs, con.1.clone()));
    }

    for _ in 0..1000 {
        let mut pulses: VecDeque<Pulse> = VecDeque::new();
        let start_pulse: Vec<Pulse> = broadcaster.broadcast(button.press());
        low_pulses += 1;
        pulses.extend(start_pulse);
        while pulses.len() > 0 {
            let pulse: Pulse = pulses.pop_front().unwrap();
            match pulse.pulse {
                PulseType::Low => {
                    low_pulses += 1;
                }
                PulseType::High => {
                    high_pulses += 1;
                }
            }
            let pulse_type: PulseType = pulse.pulse.clone();
            let pulse_dest: String = pulse.destination.clone();
            let pulse_source: String = pulse.source.clone();

            if flip_flops.contains_key(&pulse_dest) {
                if let Some(output_pulses) = flip_flops.get_mut(&pulse_dest).unwrap().relay(pulse_type) {
                    pulses.extend(output_pulses);
                }
            } else if conjunctions.contains_key(&pulse_dest) {
                let output_pulses: Vec<Pulse> = conjunctions.get_mut(&pulse_dest).unwrap().relay(pulse_type, pulse_source.clone());
                pulses.extend(output_pulses);
            }
        }
    }

    high_pulses * low_pulses
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1("broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a");
        assert_eq!(result, 32000000);
    }

    #[test]
    fn it_still_works() {
        let result = part1("broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output");
        assert_eq!(result, 11687500);
    }
}
