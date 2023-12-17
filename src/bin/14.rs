advent_of_code::solution!(14);

pub fn part_one(input: &str) -> Option<u32> {
    let mut input: Vec<Vec<_>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'O' => Some(true),
                    '#' => Some(false),
                    '.' => None,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    for i in 0..input.len() {
        for j in 0..input[0].len() {
            if input[i][j] == Some(true) {
                let mut up = 0;
                for k in 1..=i {
                    if input[i - k][j] != None {
                        break;
                    }
                    up += 1;
                }

                input[i][j] = None;
                input[i - up][j] = Some(true);
            }
        }
    }

    let weight = input
        .iter()
        .rev()
        .enumerate()
        .map(|(i, line)| {
            line.iter()
                .map(|j| if *j == Some(true) { i as u32 + 1 } else { 0 })
                .sum::<u32>()
        })
        .sum();

    Some(weight)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut grid: Vec<Vec<_>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'O' => Some(true),
                    '#' => Some(false),
                    '.' => None,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    let mut history = vec![];
    for h in 0..1000000000 {
        history.push(grid.clone());

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == Some(true) {
                    let mut up = 0;
                    for k in 1..=i {
                        if grid[i - k][j] != None {
                            break;
                        }
                        up += 1;
                    }

                    grid[i][j] = None;
                    grid[i - up][j] = Some(true);
                }
            }
        }

        for j in 0..grid[0].len() {
            for i in 0..grid.len() {
                if grid[i][j] == Some(true) {
                    let mut left = 0;
                    for k in 1..=j {
                        if grid[i][j - k] != None {
                            break;
                        }
                        left += 1;
                    }

                    grid[i][j] = None;
                    grid[i][j - left] = Some(true);
                }
            }
        }

        for i in (0..grid.len()).rev() {
            for j in 0..grid[0].len() {
                if grid[i][j] == Some(true) {
                    let mut down = 0;
                    for k in 1..=(grid.len() - i - 1) {
                        if grid[i + k][j] != None {
                            break;
                        }
                        down += 1;
                    }

                    grid[i][j] = None;
                    grid[i + down][j] = Some(true);
                }
            }
        }

        for j in (0..grid[0].len()).rev() {
            for i in 0..grid.len() {
                if grid[i][j] == Some(true) {
                    let mut right = 0;
                    for k in 1..=(grid.len() - j - 1) {
                        if grid[i][j + k] != None {
                            break;
                        }
                        right += 1;
                    }

                    grid[i][j] = None;
                    grid[i][j + right] = Some(true);
                }
            }
        }

        if let Some(index) = history.iter().position(|g| *g == grid) {
            let cycle_length = h - index + 1;
            let in_cycle = (1000000000 - index) % cycle_length;
            grid = history[index + in_cycle].clone();
            break;
        }
    }

    let weight = grid
        .iter()
        .rev()
        .enumerate()
        .map(|(i, line)| {
            line.iter()
                .map(|j| if *j == Some(true) { i as u32 + 1 } else { 0 })
                .sum::<u32>()
        })
        .sum();

    Some(weight)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
