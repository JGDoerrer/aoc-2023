use std::collections::HashMap;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let mut numbers = vec![];

    for (i, line) in input.lines().enumerate() {
        let mut j = 0;
        while j < line.len() {
            if line.chars().nth(j).unwrap().is_ascii_digit() {
                let num: String = line
                    .chars()
                    .skip(j)
                    .take_while(|c| c.is_ascii_digit())
                    .collect();
                let start_j = j;
                j += num.len();
                let num: u32 = num.parse().unwrap();
                numbers.push((i, start_j, num));
            } else {
                j += 1;
            }
        }
    }

    let mut sum = 0;
    'num_loop: for (i, j, num) in numbers {
        let len = num.ilog10() + 1;

        for di in -1isize..=1 {
            let i = (i as isize + di).clamp(0, input.lines().count() as isize) as usize;

            for dj in -1isize..=len as isize {
                let j = (j as isize + dj).clamp(0, input.lines().nth(j).unwrap().len() as isize)
                    as usize;

                if input
                    .lines()
                    .nth(i)
                    .and_then(|line| line.chars().nth(j).map(|c| c != '.' && !c.is_ascii_digit()))
                    .unwrap_or(false)
                {
                    sum += num;
                    continue 'num_loop;
                }
            }
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut numbers = vec![];

    for (i, line) in input.lines().enumerate() {
        let mut j = 0;
        while j < line.len() {
            if line.chars().nth(j).unwrap().is_ascii_digit() {
                let num: String = line
                    .chars()
                    .skip(j)
                    .take_while(|c| c.is_ascii_digit())
                    .collect();
                let start_j = j;
                j += num.len();
                let num: u32 = num.parse().unwrap();
                numbers.push((i, start_j, num));
            } else {
                j += 1;
            }
        }
    }

    let mut gears: HashMap<(usize, usize), Vec<u32>> = HashMap::new();
    'num_loop: for (i, j, num) in numbers {
        let len = num.ilog10() + 1;

        for di in -1isize..=1 {
            let i = (i as isize + di).clamp(0, input.lines().count() as isize) as usize;

            for dj in -1isize..=len as isize {
                let j = (j as isize + dj).clamp(0, input.lines().nth(j).unwrap().len() as isize)
                    as usize;

                if input
                    .lines()
                    .nth(i)
                    .and_then(|line| line.chars().nth(j).map(|c| c == '*'))
                    .unwrap_or(false)
                {
                    if let Some(gear) = gears.get_mut(&(i, j)) {
                        gear.push(num);
                    } else {
                        gears.insert((i, j), vec![num]);
                    }
                    continue 'num_loop;
                }
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
