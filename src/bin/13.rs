advent_of_code::solution!(13);

pub fn part_one(input: &str) -> Option<u32> {
    let patterns: Vec<Vec<Vec<_>>> = input
        .split("\n\n")
        .map(|pattern| {
            pattern
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|c| match c {
                            '#' => true,
                            '.' => false,
                            _ => unreachable!(),
                        })
                        .collect()
                })
                .collect()
        })
        .collect();

    let mut sum = 0;
    for pattern in patterns {
        let height = pattern.len();
        let width = pattern[0].len();

        for i in 0..height - 1 {
            let mut is_mirror = true;
            for j in 0..(i + 1).min(height - i - 1) {
                if pattern[i + j + 1] != pattern[i - j] {
                    is_mirror = false;
                }
            }

            if is_mirror {
                sum += (i + 1) * 100;
            }
        }

        for i in 0..width - 1 {
            let mut is_mirror = true;
            for j in 0..(i + 1).min(width - i - 1) {
                if pattern.iter().any(|line| line[i + j + 1] != line[i - j]) {
                    is_mirror = false;
                }
            }

            if is_mirror {
                sum += i + 1;
            }
        }
    }

    Some(sum as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let patterns: Vec<Vec<Vec<_>>> = input
        .split("\n\n")
        .map(|pattern| {
            pattern
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|c| match c {
                            '#' => true,
                            '.' => false,
                            _ => unreachable!(),
                        })
                        .collect()
                })
                .collect()
        })
        .collect();

    let mut sum = 0;
    for pattern in patterns {
        let height = pattern.len();
        let width = pattern[0].len();

        for i in 0..height - 1 {
            let mut diff = vec![vec![true; width]; height];
            for j in 0..(i + 1).min(height - i - 1) {
                diff[j] = pattern[i + j + 1]
                    .iter()
                    .zip(pattern[i - j].iter())
                    .map(|(a, b)| a == b)
                    .collect();
            }

            if diff
                .iter()
                .map(|d| d.iter().filter(|d| !**d).count())
                .sum::<usize>()
                == 1
            {
                sum += (i + 1) * 100;
            }
        }

        for i in 0..width - 1 {
            let mut diff = vec![vec![true; height]; width];
            for j in 0..(i + 1).min(width - i - 1) {
                diff[j] = pattern
                    .iter()
                    .map(|line| line[i + j + 1] == line[i - j])
                    .collect();
            }

            if diff
                .iter()
                .map(|d| d.iter().filter(|d| !**d).count())
                .sum::<usize>()
                == 1
            {
                sum += i + 1;
            }
        }
    }

    Some(sum as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(400));
    }
}
