use std::collections::HashMap;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;
    let lines = input.lines().count();
    let line_len = input.lines().next().unwrap().len();

    for (i, line) in input.lines().enumerate() {
        let mut j = 0;
        'num_loop: while j < line.len() {
            if line.chars().nth(j).unwrap().is_ascii_digit() {
                let num: String = line
                    .chars()
                    .skip(j)
                    .take_while(|c| c.is_ascii_digit())
                    .collect();
                let len = num.len();

                let num: u32 = num.parse().unwrap();

                for di in -1isize..=1 {
                    let i = (i as isize + di).clamp(0, lines as isize - 1) as usize;

                    let start_j = j.saturating_sub(1);
                    let end_j = (j + len + 1).min(line_len);

                    if input
                        .lines()
                        .nth(i)
                        .unwrap()
                        .chars()
                        .take(end_j)
                        .skip(start_j)
                        .any(|c| c != '.' && !c.is_ascii_digit())
                    {
                        sum += num;
                        j += len;
                        continue 'num_loop;
                    }
                }

                j += len;
            } else {
                j += 1;
            }
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut gears: HashMap<(usize, usize), Vec<u32>> = HashMap::new();
    let lines = input.lines().count();
    let line_len = input.lines().next().unwrap().len();

    for (i, line) in input.lines().enumerate() {
        let mut j = 0;
        'num_loop: while j < line.len() {
            if line.chars().nth(j).unwrap().is_ascii_digit() {
                let num: String = line
                    .chars()
                    .skip(j)
                    .take_while(|c| c.is_ascii_digit())
                    .collect();
                let len = num.len();

                let num: u32 = num.parse().unwrap();

                for di in -1isize..=1 {
                    let i = (i as isize + di).clamp(0, lines as isize - 1) as usize;

                    let start_j = j.saturating_sub(1);
                    let end_j = (j + len + 1).min(line_len);

                    if let Some((char_j, _)) = input
                        .lines()
                        .nth(i)
                        .unwrap()
                        .chars()
                        .enumerate()
                        .take(end_j)
                        .skip(start_j)
                        .filter(|(_, c)| *c == '*')
                        .next()
                    {
                        if let Some(gear) = gears.get_mut(&(i, char_j)) {
                            gear.push(num);
                        } else {
                            gears.insert((i, char_j), vec![num]);
                        }
                        j += len;
                        continue 'num_loop;
                    }
                }

                j += len;
            } else {
                j += 1;
            }
        }
    }

    let sum = gears
        .into_iter()
        .map(|(_, gears)| {
            if gears.len() == 2 {
                gears[0] * gears[1]
            } else {
                0
            }
        })
        .sum();

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
