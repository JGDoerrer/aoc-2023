advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let (winning, numbers) = line[9..].split_once('|').unwrap();
                let winning = winning
                    .split_whitespace()
                    .map(|n| n.parse().unwrap())
                    .collect::<Vec<u32>>();
                let numbers = numbers
                    .split_whitespace()
                    .map(|n| n.parse().unwrap())
                    .collect::<Vec<u32>>();

                (1 << numbers.iter().filter(|n| winning.contains(n)).count()) as u32 / 2
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let winnings = input
        .lines()
        .map(|line| {
            let (winning, numbers) = line[9..].split_once('|').unwrap();
            let winning = winning
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect::<Vec<u32>>();
            let numbers = numbers
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect::<Vec<u32>>();

            numbers.iter().filter(|n| winning.contains(n)).count() as u32
        })
        .collect::<Vec<_>>();

    let mut copies = vec![1; input.lines().count()];

    for i in 0..input.lines().count() {
        let winnings = winnings[i];
        let amount = copies[i];

        for di in 0..winnings as usize {
            copies[i + di + 1] += amount;
        }
    }

    Some(copies.into_iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
