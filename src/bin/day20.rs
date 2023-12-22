use std::collections::{HashMap, VecDeque};

use aoc_rs_2023::*;

use itertools::Itertools;

fn main() {
    println!("Part 1: {}", part_1(input!()));
    //println!("Part 2: {}", part_2(input!()));
}

fn part_1(input: &str) -> u32 {
    let mut circuit = parse_circuit(input);

    let mut high_count = 0;
    let mut low_count = 0;
    let start = circuit.get("broadcaster").unwrap().get_label().to_owned();

    for _ in 0..1000 {
        let mut queue = VecDeque::from_iter([("button".to_owned(), Pulse::Low, start.to_owned())]);

        while let Some((source, pulse, target)) = queue.pop_front() {
            match pulse {
                Pulse::High => high_count += 1,
                Pulse::Low => low_count += 1,
            }

            if let Some(module) = circuit.get_mut(&target) {
                if let Some(pulse) = module.process_pulse(pulse, &source) {
                    for next in module.get_targets() {
                        queue.push_back((target.to_owned(), pulse, next.to_owned()));
                    }
                }
            }
        }
    }

    high_count * low_count
}

/*fn part_2(input: &str) -> u32 {
    let mut circuit = parse_circuit(input);
    let start = circuit.get("broadcaster").unwrap().get_label().to_owned();

    for i in 1.. {
        //println!("============={}=============", i);
        let mut queue = VecDeque::from_iter([("button".to_owned(), Pulse::Low, start.to_owned())]);

        while let Some((source, pulse, target)) = queue.pop_front() {
            //println!("{} --{:?}--> {}", source, pulse, target);

            if target == "rx" {
                println!("rx <- {:?}", pulse);
                if pulse == Pulse::High {
                    return i;
                }
            }

            if let Some(module) = circuit.get_mut(&target) {
                if let Some(pulse) = module.process_pulse(pulse, &source) {
                    for next in module.get_targets() {
                        queue.push_back((target.to_owned(), pulse, next.to_owned()));
                    }
                }
            }
        }
    }

    0
}*/

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Pulse {
    High,
    Low,
}

enum Module {
    Broadcaster {
        label: String,
        targets: Vec<String>,
    },
    FlipFlop {
        label: String,
        targets: Vec<String>,
        is_on: bool,
    },
    Conjunction {
        label: String,
        targets: Vec<String>,
        memory: HashMap<String, Pulse>,
    },
}

impl Module {
    fn process_pulse(&mut self, pulse: Pulse, source: &str) -> Option<Pulse> {
        match self {
            Module::Broadcaster { .. } => Some(pulse),

            Module::FlipFlop { is_on, .. } => {
                if pulse == Pulse::High {
                    return None;
                }

                *is_on ^= true;
                let pulse = if *is_on { Pulse::High } else { Pulse::Low };

                Some(pulse)
            }

            Module::Conjunction { memory, .. } => {
                let source_memory = memory.get_mut(source).unwrap();
                *source_memory = pulse;

                let pulse = if memory.values().all(|&p| p == Pulse::High) {
                    Pulse::Low
                } else {
                    Pulse::High
                };

                Some(pulse)
            }
        }
    }

    fn get_targets(&self) -> &[String] {
        match self {
            Module::Broadcaster { targets, .. } => targets,
            Module::FlipFlop { targets, .. } => targets,
            Module::Conjunction { targets, .. } => targets,
        }
    }

    fn get_label(&self) -> &String {
        match self {
            Module::Broadcaster { label, .. } => label,
            Module::FlipFlop { label, .. } => label,
            Module::Conjunction { label, .. } => label,
        }
    }
}

fn parse_circuit(input: &str) -> HashMap<String, Module> {
    let mut circuit = input
        .lines()
        .map(|line| {
            let (module, targets) = line.split_once(" -> ").unwrap();
            let label = module
                .trim_start_matches(|c| c == '%' || c == '&')
                .to_string();
            let targets = targets
                .split(',')
                .map(|s| s.trim().to_string())
                .collect_vec();

            let new_module = match module.chars().next() {
                Some('%') => Module::FlipFlop {
                    label,
                    targets,
                    is_on: false,
                },
                Some('&') => Module::Conjunction {
                    label,
                    targets,
                    memory: HashMap::new(),
                },
                Some(_) => Module::Broadcaster { label, targets },
                None => unreachable!(),
            };

            (new_module.get_label().clone(), new_module)
        })
        .collect::<HashMap<String, Module>>();

    // Initialize "conjunction" modules' memories once the all links are known:
    let links = circuit
        .iter()
        .flat_map(|(source, module)| {
            module
                .get_targets()
                .iter()
                .map(|target| (source.to_owned(), target.to_owned()))
        })
        .collect_vec();

    for (source, target) in links {
        let target = circuit.get_mut(target.as_str());

        if let Some(mut target) = target {
            match &mut target {
                Module::Conjunction { memory, .. } => {
                    memory.insert(source.to_owned(), Pulse::Low);
                }
                _ => continue,
            }
        }
    }

    circuit
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sample() {
        assert_eq!(part_1(sample1!()), 32000000);
        assert_eq!(part_1(sample2!()), 11687500);
    }
}
