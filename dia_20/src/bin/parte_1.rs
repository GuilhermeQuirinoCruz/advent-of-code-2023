use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
struct Module {
    m_type: char,
    destinations: Vec<String>,
}

fn main() {
    // let input: &str = include_str!("./exemplo_1.txt");
    // let input: &str = include_str!("./exemplo_2.txt");
    let input: &str = include_str!("./input.txt");

    let mut modules: HashMap<String, Module> = HashMap::new();
    let mut conjunction_inputs: HashMap<String, HashMap<String, bool>> = HashMap::new();
    let mut flip_flops: HashMap<String, bool> = HashMap::new();
    for line in input.lines() {
        let (type_and_name, destinations): (&str, &str) = line.split_once(" -> ").unwrap();

        let m_type: char = type_and_name.chars().next().unwrap();

        let module: Module = Module {
            m_type: m_type,
            destinations: destinations
                .split(", ")
                .map(|dest| dest.to_string())
                .collect(),
        };

        let name: String = if m_type != 'b' {
            (&type_and_name[1..]).to_string()
        } else {
            type_and_name.to_string()
        };

        modules.insert(name.to_string(), module);

        if m_type == '&' {
            conjunction_inputs.insert(name, HashMap::new());
        } else if m_type == '%' {
            flip_flops.insert(name, false);
        }
    }

    for (name, module) in &modules {
        for dest in &module.destinations {
            match modules.get(dest) {
                Some(target) => {
                    if target.m_type != '&' {
                        continue;
                    }

                    conjunction_inputs
                        .get_mut(dest)
                        .unwrap()
                        .insert(name.to_string(), false);
                }
                None => (),
            }
        }
    }

    let mut high_pulses_sent: u32 = 0;
    let mut low_pulses_sent: u32 = 0;
    let mut times_to_push: u32 = 1000;

    while times_to_push > 0 {
        times_to_push -= 1;

        let mut signals: VecDeque<(String, String, bool)> =
            VecDeque::from([("".to_string(), "broadcaster".to_string(), false)]);
        while signals.len() > 0 {
            let (sender, receiver, pulse) = signals.pop_front().unwrap();

            if pulse {
                high_pulses_sent += 1;
            } else {
                low_pulses_sent += 1;
            }

            if !modules.contains_key(&receiver) {
                continue;
            }

            let receiver_module: &Module = modules.get(&receiver).unwrap();
            let pulse_to_send: bool;
            match receiver_module.m_type {
                'b' => {
                    pulse_to_send = pulse;
                }
                '%' => {
                    if pulse {
                        continue;
                    }

                    pulse_to_send = !(*flip_flops.get(&receiver).unwrap());
                    flip_flops.insert(receiver.to_string(), pulse_to_send);
                }
                '&' => {
                    conjunction_inputs
                        .get_mut(&receiver)
                        .unwrap()
                        .insert(sender.to_string(), pulse);

                    pulse_to_send = !conjunction_inputs
                        .get(&receiver)
                        .unwrap()
                        .values()
                        .all(|high| *high);
                }
                _ => pulse_to_send = false,
            }

            for module in &receiver_module.destinations {
                signals.push_back((receiver.to_string(), module.to_string(), pulse_to_send));
            }
        }
    }

    println!("{}", low_pulses_sent * high_pulses_sent);
}
