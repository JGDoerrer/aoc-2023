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
        let first_win = (0..=time)
            .find(|start| (time - start) * start > target)
            .unwrap();
        let wins = time + 1 - first_win * 2;

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

    let wins = 2
        * ((((time * time / 4 - target - 1) as f32).sqrt() + (time % 2) as f32 / 2.0).floor())
            as u64
        + 1
        - time % 2;

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
