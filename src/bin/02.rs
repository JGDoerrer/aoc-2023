advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    Some(
        input
            .lines()
            .map(|line| {
                let (start, rounds) = line.split_once(':').unwrap();
                let id: u32 = start[5..].parse().unwrap();

                let mut possible = true;
                'round_loop: for round in rounds.split(';') {
                    for amount in round.split(',') {
                        let (amount, color) = amount.trim().split_once(' ').unwrap();
                        let amount: u32 = amount.parse().unwrap();

                        if color == "red" && amount > max_red
                            || color == "blue" && amount > max_blue
                            || color == "green" && amount > max_green
                        {
                            possible = false;
                            break 'round_loop;
                        }
                    }
                }

                if possible {
                    id
                } else {
                    0
                }
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let (_, rounds) = line.split_once(':').unwrap();

                let mut min_red = 0;
                let mut min_blue = 0;
                let mut min_green = 0;

                rounds.split(';').for_each(|round| {
                    round.split(',').for_each(|amount| {
                        let (amount, color) = amount.trim().split_once(' ').unwrap();
                        let amount: u32 = amount.parse().unwrap();
                        if color == "red" && amount > min_red {
                            min_red = amount;
                        }

                        if color == "blue" && amount > min_blue {
                            min_blue = amount;
                        }

                        if color == "green" && amount > min_green {
                            min_green = amount;
                        }
                    });
                });

                min_red * min_blue * min_green
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
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
