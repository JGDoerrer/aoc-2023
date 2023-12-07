advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u32> {
    let mut inputs: Vec<([u32; 5], u32)> = input
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_at(5);
            (
                hand.chars()
                    .map(|c| match c {
                        'T' => 10,
                        'J' => 11,
                        'Q' => 12,
                        'K' => 13,
                        'A' => 14,
                        c => c.to_digit(10).unwrap(),
                    })
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap(),
                bid[1..].parse().unwrap(),
            )
        })
        .collect();

    inputs.sort_by_cached_key(|(hand, _)| {
        let mut counts = [0; 13];

        for card in hand {
            counts[*card as usize - 2] += 1;
        }

        let mut counts: [_; 13] = counts
            .into_iter()
            .enumerate()
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        counts.sort_by_key(|(_, count)| 5 - count);
        let max_count = counts[0].1;
        let second_max_count = counts[1].1;

        let mut value = match max_count {
            5 => 6,
            4 => 5,
            3 => match second_max_count {
                2 => 4,
                _ => 3,
            },
            2 => match second_max_count {
                2 => 2,
                _ => 1,
            },
            _ => 0,
        };

        value *= 6;
        value += hand[0] as usize - 1;
        value *= 13;
        value += hand[1] as usize - 1;
        value *= 13;
        value += hand[2] as usize - 1;
        value *= 13;
        value += hand[3] as usize - 1;
        value *= 13;
        value += hand[4] as usize - 1;

        usize::MAX - value
    });

    let len = inputs.len();
    let winnings = inputs
        .into_iter()
        .enumerate()
        .map(|(i, (_, bid))| (len - i) as u32 * bid)
        .sum();

    Some(winnings)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut inputs: Vec<([u32; 5], u32)> = input
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_at(5);
            (
                hand.chars()
                    .map(|c| match c {
                        'T' => 10,
                        'J' => 1,
                        'Q' => 12,
                        'K' => 13,
                        'A' => 14,
                        c => c.to_digit(10).unwrap(),
                    })
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap(),
                bid[1..].parse().unwrap(),
            )
        })
        .collect();

    inputs.sort_by_cached_key(|(hand, _)| {
        let mut counts = [0; 14];

        for card in hand {
            counts[*card as usize - 1] += 1;
        }

        let jokers = counts[0];

        let mut counts: [_; 14] = counts
            .into_iter()
            .enumerate()
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        counts.sort_by_key(|(_, count)| 5 - count);
        let (max_count, second_max_count) = if counts[0].0 != 0 {
            if counts[1].0 != 0 {
                (counts[0].1, counts[1].1)
            } else {
                (counts[0].1, counts[2].1)
            }
        } else {
            (counts[1].1, counts[2].1)
        };

        let mut value = match max_count + jokers {
            5 => 6,
            4 => 5,
            3 => match second_max_count {
                2 => 4,
                _ => 3,
            },
            2 => match second_max_count {
                2 => 2,
                _ => 1,
            },
            _ => 0,
        };

        value *= 6;
        value += hand[0] as usize - 1;
        value *= 13;
        value += hand[1] as usize - 1;
        value *= 13;
        value += hand[2] as usize - 1;
        value *= 13;
        value += hand[3] as usize - 1;
        value *= 13;
        value += hand[4] as usize - 1;

        usize::MAX - value
    });

    let len = inputs.len();
    let winnings = inputs
        .into_iter()
        .enumerate()
        .map(|(i, (_, bid))| (len - i) as u32 * bid)
        .sum();

    Some(winnings)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
