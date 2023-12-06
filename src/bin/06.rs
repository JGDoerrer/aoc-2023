advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let times: Vec<u32> = input
        .lines()
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|i| i.parse().unwrap())
        .collect();

    let distances: Vec<u32> = input
        .lines()
        .nth(1)
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|i| i.parse().unwrap())
        .collect();

    let mut product = 1;
    for (time, target) in times.into_iter().zip(distances.into_iter()) {
        let mut wins = 0;
        for start in 0..=time {
            let distance = (time - start) * start;

            if distance > target {
                wins += 1;
            }
        }
        product *= wins;
    }

    Some(product)
}

pub fn part_two(input: &str) -> Option<u64> {
    let time: u64 = input
        .lines()
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .parse()
        .unwrap();

    let target: u64 = input
        .lines()
        .nth(1)
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .parse()
        .unwrap();

    let mut wins = 0;
    for start in 0..=time {
        let distance = (time - start) * start;

        if distance > target {
            wins += 1;
        }
    }

    Some(wins)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
