use std::collections::HashMap;

advent_of_code::solution!(19);

pub fn part_one(input: &str) -> Option<u32> {
    #[derive(Debug)]
    enum Workflow {
        ConditionSend(usize, bool, u32, String),
        Condition(usize, bool, u32, bool),
        ImmSend(String),
        Imm(bool),
    }

    let (workflows, parts) = input.split_once("\n\n").unwrap();
    let parts: Vec<_> = parts
        .lines()
        .map(|line| {
            let values: [_; 4] = line
                .trim_matches(|c| c == '{' || c == '}')
                .split(',')
                .map(|p| p[2..].parse::<u32>().unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
            values
        })
        .collect();
    let workflows: HashMap<_, _> = workflows
        .lines()
        .map(|line| {
            let name: String = line.chars().take_while(|c| c.is_alphabetic()).collect();
            let rules: Vec<_> = line[name.len()..]
                .trim_matches(|c| c == '{' || c == '}')
                .split(',')
                .map(|p| {
                    if let Some((condition, result)) = p.split_once(':') {
                        let var = match condition.chars().next().unwrap() {
                            'x' => 0,
                            'm' => 1,
                            'a' => 2,
                            's' => 3,
                            _ => unreachable!(),
                        };
                        let less = condition.chars().nth(1).unwrap() == '<';
                        let value = condition[2..].parse().unwrap();

                        match result {
                            "A" => Workflow::Condition(var, less, value, true),
                            "R" => Workflow::Condition(var, less, value, false),
                            res => Workflow::ConditionSend(var, less, value, res.to_string()),
                        }
                    } else {
                        match p {
                            "A" => Workflow::Imm(true),
                            "R" => Workflow::Imm(false),
                            _ => Workflow::ImmSend(p.to_string()),
                        }
                    }
                })
                .collect();
            (name, rules)
        })
        .collect();

    Some(
        parts
            .into_iter()
            .map(|part| {
                let mut workflow = &workflows["in"];
                let accepted = 'l: loop {
                    for rule in workflow {
                        match rule {
                            Workflow::ConditionSend(var, less, value, next) => match less {
                                true => {
                                    if part[*var] < *value {
                                        workflow = &workflows[next];
                                        break;
                                    }
                                }
                                false => {
                                    if part[*var] > *value {
                                        workflow = &workflows[next];
                                        break;
                                    }
                                }
                            },
                            Workflow::Condition(var, less, value, accept) => match less {
                                true => {
                                    if part[*var] < *value {
                                        break 'l *accept;
                                    }
                                }
                                false => {
                                    if part[*var] > *value {
                                        break 'l *accept;
                                    }
                                }
                            },
                            Workflow::ImmSend(next) => {
                                workflow = &workflows[next];
                                break;
                            }
                            Workflow::Imm(accept) => break 'l *accept,
                        }
                    }
                };

                if accepted {
                    part.into_iter().sum::<u32>()
                } else {
                    0
                }
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    #[derive(Debug)]
    enum Workflow {
        ConditionSend(usize, bool, u32, String),
        Condition(usize, bool, u32, bool),
        ImmSend(String),
        Imm(bool),
    }

    let (workflows, _) = input.split_once("\n\n").unwrap();
    let workflows: HashMap<_, _> = workflows
        .lines()
        .map(|line| {
            let name: String = line.chars().take_while(|c| c.is_alphabetic()).collect();
            let rules: Vec<_> = line[name.len()..]
                .trim_matches(|c| c == '{' || c == '}')
                .split(',')
                .map(|p| {
                    if let Some((condition, result)) = p.split_once(':') {
                        let var = match condition.chars().next().unwrap() {
                            'x' => 0,
                            'm' => 1,
                            'a' => 2,
                            's' => 3,
                            _ => unreachable!(),
                        };
                        let less = condition.chars().nth(1).unwrap() == '<';
                        let value = condition[2..].parse().unwrap();

                        match result {
                            "A" => Workflow::Condition(var, less, value, true),
                            "R" => Workflow::Condition(var, less, value, false),
                            res => Workflow::ConditionSend(var, less, value, res.to_string()),
                        }
                    } else {
                        match p {
                            "A" => Workflow::Imm(true),
                            "R" => Workflow::Imm(false),
                            _ => Workflow::ImmSend(p.to_string()),
                        }
                    }
                })
                .collect();
            (name, rules)
        })
        .collect();

    let mut ranges = vec![("in", [1..=4000, 1..=4000, 1..=4000, 1..=4000])];

    let mut sum = 0;
    'l: while let Some((wf, mut range)) = ranges.pop() {
        let workflow = &workflows[wf];
        let mut accepted = false;
        for rule in workflow {
            match rule {
                Workflow::ConditionSend(var, less, value, next) => match less {
                    true => {
                        let val_range = &range[*var];
                        if val_range.end() < value {
                            ranges.push((next, range.clone()));
                            continue 'l;
                        } else if val_range.start() >= value {
                        } else {
                            let a = *val_range.start()..=(*value - 1);
                            let b = *value..=*val_range.end();
                            let mut new_range = range.clone();
                            if a.start() <= a.end() {
                                new_range[*var] = a;
                                ranges.push((next, new_range));
                            }
                            range[*var] = b;
                        }
                    }
                    false => {
                        let val_range = &range[*var];
                        if val_range.start() > value {
                            ranges.push((next, range.clone()));
                            continue 'l;
                        } else if val_range.end() <= value {
                        } else {
                            let a = *val_range.start()..=*value;
                            let b = (*value + 1)..=*val_range.end();
                            let mut new_range = range.clone();
                            if b.start() <= b.end() {
                                new_range[*var] = b;
                                ranges.push((next, new_range));
                            }
                            range[*var] = a;
                        }
                    }
                },
                Workflow::Condition(var, less, value, accept) => match less {
                    true => {
                        let val_range = &range[*var];
                        if val_range.end() < value {
                            accepted = *accept;
                        } else if val_range.start() >= value {
                        } else {
                            let a = *val_range.start()..=(*value - 1);
                            let b = *value..=*val_range.end();
                            let mut new_range = range.clone();
                            if a.start() <= a.end() {
                                new_range[*var] = a;
                                if *accept {
                                    sum += new_range
                                        .into_iter()
                                        .map(|r| r.count() as u64)
                                        .product::<u64>();
                                }
                            }
                            range[*var] = b;
                        }
                    }
                    false => {
                        let val_range = &range[*var];
                        if val_range.start() > value {
                            accepted = *accept;
                        } else if val_range.end() <= value {
                        } else {
                            let a = *val_range.start()..=*value;
                            let b = (*value + 1)..=*val_range.end();
                            let mut new_range = range.clone();
                            if b.start() <= b.end() {
                                new_range[*var] = b;
                                if *accept {
                                    sum += new_range
                                        .into_iter()
                                        .map(|r| r.count() as u64)
                                        .product::<u64>();
                                }
                            }
                            range[*var] = a;
                        }
                    }
                },
                Workflow::ImmSend(next) => {
                    ranges.push((next, range.clone()));
                    continue 'l;
                }
                Workflow::Imm(accept) => accepted = *accept,
            }
        }

        if accepted {
            sum += range.into_iter().map(|r| r.count() as u64).product::<u64>();
        }
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(19114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(167409079868000));
    }
}
