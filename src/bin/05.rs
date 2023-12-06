advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let mut parts = input.trim().split("\n\n");

    let seeds: Vec<u64> = parts
        .next()
        .unwrap()
        .split(' ')
        .skip(1)
        .map(|i| i.parse().unwrap())
        .collect();

    let parts = parts
        .map(|part| {
            part.lines()
                .skip(1)
                .map(|line| {
                    line.splitn(3, ' ')
                        .map(|r| r.parse::<u64>().unwrap())
                        .collect::<Vec<u64>>()
                        .try_into()
                        .unwrap()
                })
                .collect::<Vec<[u64; 3]>>()
        })
        .collect::<Vec<_>>();

    let min = seeds
        .into_iter()
        .map(|seed| {
            let mut seed = seed;
            'part_loop: for part in &parts {
                for [dest, source, len] in part {
                    if seed >= *source && seed < source + len {
                        seed = seed + dest - source;
                        continue 'part_loop;
                    }
                }
            }

            seed
        })
        .min()
        .unwrap();

    Some(min)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut parts = input.trim().split("\n\n");

    let seeds: Vec<u64> = parts
        .next()
        .unwrap()
        .split(' ')
        .skip(1)
        .map(|i| i.parse().unwrap())
        .collect();

    let seeds: Vec<_> = seeds
        .iter()
        .enumerate()
        .filter(|(i, _)| *i % 2 == 0)
        .map(|(_, n)| *n)
        .zip(
            seeds
                .iter()
                .enumerate()
                .filter(|(i, _)| *i % 2 == 1)
                .map(|(_, n)| *n),
        )
        .collect();

    let parts = parts
        .map(|part| {
            part.lines()
                .skip(1)
                .map(|line| {
                    line.splitn(3, ' ')
                        .map(|r| r.parse::<u64>().unwrap())
                        .collect::<Vec<u64>>()
                        .try_into()
                        .unwrap()
                })
                .collect::<Vec<[u64; 3]>>()
        })
        .collect::<Vec<_>>();

    let min = seeds
        .into_iter()
        .map(|(seed_start, seed_len)| {
            let mut ranges = vec![(seed_start, seed_len)];

            for part in &parts {
                let mut new_ranges = vec![];
                while let Some((seed_start, seed_len)) = ranges.pop() {
                    let mut mapped = false;
                    let seed_end = seed_start + seed_len;
                    for [dest, source_start, len] in part.clone() {
                        let source_end = source_start + len;

                        if source_start >= seed_end || source_end <= seed_start {
                            continue;
                        }

                        mapped = true;

                        let inside_range = (source_start.max(seed_start), source_end.min(seed_end));

                        new_ranges.push((
                            inside_range.0 + dest - source_start,
                            inside_range.1 - inside_range.0,
                        ));

                        if seed_start < source_start {
                            let outside_range = (seed_start, source_start - seed_start);
                            ranges.push(outside_range);
                        }

                        if seed_end > source_end {
                            let outside_range = (source_end, seed_end - source_end);
                            ranges.push(outside_range);
                        }

                        break;
                    }

                    if !mapped {
                        new_ranges.push((seed_start, seed_len));
                    }
                }

                ranges = new_ranges;
            }

            ranges.iter().map(|(start, _)| *start).min().unwrap()
        })
        .min()
        .unwrap();

    Some(min)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
