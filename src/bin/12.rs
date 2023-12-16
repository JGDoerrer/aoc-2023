use std::collections::HashMap;

advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<_> = input
        .lines()
        .map(|line| {
            let (springs, counts) = line.split_once(' ').unwrap();
            let springs: Vec<_> = springs
                .chars()
                .map(|c| match c {
                    '#' => Some(true),
                    '.' => Some(false),
                    '?' => None,
                    _ => unreachable!(),
                })
                .collect();
            let counts: Vec<_> = counts.split(',').map(|c| c.parse().unwrap()).collect();
            (springs, counts)
        })
        .collect();

    fn solve_rec((line, counts): (Vec<Option<bool>>, Vec<u32>)) -> u32 {
        let next_unkown = line.iter().position(|s| s.is_none());
        if let Some(index) = next_unkown {
            let mut line_true = line.clone();
            line_true[index] = Some(true);
            let mut sum = solve_rec((line_true, counts.clone()));
            let mut line_false = line.clone();
            line_false[index] = Some(false);
            sum += solve_rec((line_false, counts));
            sum
        } else {
            let mut line_counts = vec![];

            let mut current_count = 0;
            for c in &line {
                if c.unwrap() {
                    current_count += 1;
                } else {
                    if current_count > 0 {
                        line_counts.push(current_count);
                    }
                    current_count = 0;
                }
            }
            if current_count > 0 {
                line_counts.push(current_count);
            }

            if line_counts == counts {
                1
            } else {
                0
            }
        }
    }

    Some(lines.into_iter().map(solve_rec).sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines: Vec<_> = input
        .trim()
        .lines()
        .map(|line| {
            let (springs, counts) = line.split_once(' ').unwrap();
            let mut springs: Vec<_> = springs
                .chars()
                .map(|c| match c {
                    '#' => Some(true),
                    '.' => Some(false),
                    '?' => None,
                    _ => unreachable!(),
                })
                .collect();
            springs.push(None);
            springs = springs.repeat(5);
            springs.pop();
            let counts = counts
                .split(',')
                .map(|c| c.parse().unwrap())
                .collect::<Vec<_>>()
                .repeat(5);
            (springs, counts)
        })
        .collect();

    fn solve_rec(
        cache: &mut HashMap<(Vec<Option<bool>>, Vec<u32>), u64>,
        (line, counts): (Vec<Option<bool>>, Vec<u32>),
    ) -> u64 {
        let mut line_counts = vec![];

        let mut current_count = 0;
        let mut elements_to_skip = 0;
        let mut counts_to_skip = 0;
        let mut all = true;
        for c in &line {
            if *c == Some(true) {
                current_count += 1;
                elements_to_skip += 1;
            } else if *c == Some(false) {
                if current_count > 0 {
                    line_counts.push(current_count);
                    counts_to_skip += 1;
                }
                elements_to_skip += 1;
                current_count = 0;
            } else {
                all = false;
                break;
            }
        }

        let ends_with_false = current_count == 0;
        if current_count > 0 {
            line_counts.push(current_count);
        }

        if all && counts == line_counts {
            return 1;
        }
        if all && counts != line_counts {
            return 0;
        }

        if (ends_with_false && !counts.starts_with(&line_counts[0..line_counts.len()]))
            || (line_counts.len() > 0
                && (!counts.starts_with(&line_counts[0..line_counts.len() - 1])))
            || line_counts.len() > counts.len()
        {
            return 0;
        }

        let (line, counts) =
            if ends_with_false && counts.starts_with(&line_counts[0..line_counts.len()]) {
                let line: Vec<_> = line.into_iter().skip(elements_to_skip).collect();
                let counts: Vec<_> = counts.into_iter().skip(counts_to_skip).collect();

                (line, counts)
            } else {
                (line, counts)
            };

        if let Some(count) = cache.get(&(line.clone(), counts.clone())) {
            return *count;
        }

        if line.len() == 0 && counts.len() == 0 {
            return 1;
        }

        let next_unkown = line.iter().position(|s| s.is_none());

        if let Some(index) = next_unkown {
            let mut sum = 0;

            let mut line_true = line.clone();
            line_true[index] = Some(true);
            sum += solve_rec(cache, (line_true, counts.clone()));

            let mut line_false = line.clone();
            line_false[index] = Some(false);
            sum += solve_rec(cache, (line_false, counts.clone()));

            cache.insert((line, counts), sum);
            sum
        } else {
            if line_counts == counts {
                1
            } else {
                0
            }
        }
    }

    Some(
        lines
            .into_iter()
            .map(|s| {
                let mut cache = HashMap::new();
                solve_rec(&mut cache, s)
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(525152));
    }
}
