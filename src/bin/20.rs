use std::collections::{HashMap, VecDeque};

advent_of_code::solution!(20);

#[derive(Debug)]
enum ModuleType {
    FlipFlop,
    Conjunction,
    Broadcast,
    Output,
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut modules: HashMap<_, _> = input
        .lines()
        .map(|line| {
            let (name, connection) = line.split_once(" -> ").unwrap();

            let typ = if name.starts_with('%') {
                ModuleType::FlipFlop
            } else if name.starts_with('&') {
                ModuleType::Conjunction
            } else if name == "broadcaster" {
                ModuleType::Broadcast
            } else {
                unreachable!()
            };

            let name = name.trim_start_matches(['%', '&']);

            let connections: Vec<_> = connection.split(',').map(|s| s.trim()).collect();

            (name, (typ, connections))
        })
        .collect();

    let mut new_modules = Vec::new();

    for (_, connections) in modules.values() {
        for c in connections {
            if !modules.contains_key(c) {
                new_modules.push((*c, (ModuleType::Output, Vec::new())));
            }
        }
    }

    for (n, m) in new_modules {
        modules.insert(n, m);
    }

    let mut flipflops: HashMap<_, _> = modules
        .iter()
        .filter(|(_, (t, _))| matches!(t, ModuleType::FlipFlop))
        .map(|(k, _)| (*k, false))
        .collect();

    let mut conjunctions: HashMap<_, _> = modules
        .iter()
        .filter(|(_, (t, _))| matches!(t, ModuleType::Conjunction))
        .map(|(k, _)| {
            let inputs: HashMap<_, _> = modules
                .iter()
                .filter(|(_, (_, c))| c.contains(k))
                .map(|(n, _)| (*n, false))
                .collect();
            (*k, inputs)
        })
        .collect();

    let mut low_pulses = 0;
    let mut high_pulses = 0;

    for _ in 0..1000 {
        let mut signals = VecDeque::new();

        low_pulses += 1;
        for m in &modules.get("broadcaster").unwrap().1 {
            signals.push_back((*m, "broadcaster", false));
        }

        while let Some((name, from, high)) = signals.pop_front() {
            if high {
                high_pulses += 1;
            } else {
                low_pulses += 1;
            }

            // dbg!(from, name, high);

            let (typ, connections) = modules.get(name).unwrap();

            if matches!(typ, ModuleType::FlipFlop) {
                if !high {
                    let flipflop = *flipflops.get(name).unwrap();
                    flipflops.insert(name, !flipflop);

                    for c in connections {
                        signals.push_back((*c, name, !flipflop));
                    }
                }
            } else if matches!(typ, ModuleType::Conjunction) {
                let inputs = conjunctions.get_mut(name).unwrap();

                inputs.insert(&from, high);

                if inputs.values().all(|v| *v) {
                    for c in connections {
                        signals.push_back((*c, name, false));
                    }
                } else {
                    for c in connections {
                        signals.push_back((*c, name, true));
                    }
                }
            }
        }
    }

    dbg!(low_pulses, high_pulses);
    Some(low_pulses * high_pulses)
}

pub fn part_two(input: &str) -> Option<u32> {
    // let mut modules: HashMap<_, _> = input
    //     .lines()
    //     .map(|line| {
    //         let (name, connection) = line.split_once(" -> ").unwrap();

    //         let typ = if name.starts_with('%') {
    //             ModuleType::FlipFlop
    //         } else if name.starts_with('&') {
    //             ModuleType::Conjunction
    //         } else if name == "broadcaster" {
    //             ModuleType::Broadcast
    //         } else {
    //             unreachable!()
    //         };

    //         let name = name.trim_start_matches(['%', '&']);

    //         let connections: Vec<_> = connection.split(',').map(|s| s.trim()).collect();

    //         (name, (typ, connections))
    //     })
    //     .collect();

    // let mut new_modules = Vec::new();

    // for (_, connections) in modules.values() {
    //     for c in connections {
    //         if !modules.contains_key(c) {
    //             new_modules.push((*c, (ModuleType::Output, Vec::new())));
    //         }
    //     }
    // }

    // for (n, m) in new_modules {
    //     modules.insert(n, m);
    // }

    // let mut flipflops: HashMap<_, _> = modules
    //     .iter()
    //     .filter(|(_, (t, _))| matches!(t, ModuleType::FlipFlop))
    //     .map(|(k, _)| (*k, false))
    //     .collect();

    // let mut conjunctions: HashMap<_, _> = modules
    //     .iter()
    //     .filter(|(_, (t, _))| matches!(t, ModuleType::Conjunction))
    //     .map(|(k, _)| {
    //         let inputs: HashMap<_, _> = modules
    //             .iter()
    //             .filter(|(_, (_, c))| c.contains(k))
    //             .map(|(n, _)| (*n, false))
    //             .collect();
    //         (*k, inputs)
    //     })
    //     .collect();

    // let to_rx = modules
    //     .iter()
    //     .filter(|(_, (_, c))| c.contains(&"rx"))
    //     .next()
    //     .unwrap();

    // dbg!(to_rx);

    // let mut i = 0;
    // loop {
    //     let mut signals = VecDeque::new();

    //     for m in &modules.get("broadcaster").unwrap().1 {
    //         signals.push_back((*m, "broadcaster", false));
    //     }

    //     while let Some((name, from, high)) = signals.pop_front() {
    //         // dbg!(from, name, high);

    //         if name == "rx" && !high {
    //             return Some(i);
    //         }

    //         let (typ, connections) = modules.get(name).unwrap();

    //         if matches!(typ, ModuleType::FlipFlop) {
    //             if !high {
    //                 let flipflop = *flipflops.get(name).unwrap();
    //                 flipflops.insert(name, !flipflop);

    //                 for c in connections {
    //                     signals.push_back((*c, name, !flipflop));
    //                 }
    //             }
    //         } else if matches!(typ, ModuleType::Conjunction) {
    //             let inputs = conjunctions.get_mut(name).unwrap();

    //             inputs.insert(&from, high);

    //             if inputs.values().all(|v| *v) {
    //                 for c in connections {
    //                     signals.push_back((*c, name, false));
    //                 }
    //             } else {
    //                 for c in connections {
    //                     signals.push_back((*c, name, true));
    //                 }
    //             }
    //         }
    //     }
    // }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11687500));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
