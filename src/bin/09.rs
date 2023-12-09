advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<i32> {
    Some(
        input
            .lines()
            .map(|line| {
                let mut numbers = line
                    .split(' ')
                    .map(|n| n.parse().unwrap())
                    .collect::<Vec<i32>>();

                let mut sum = 0;
                while numbers.iter().any(|n| *n != 0) {
                    sum += numbers.last().copied().unwrap();

                    numbers = numbers
                        .iter()
                        .zip(numbers.iter().skip(1))
                        .map(|(a, b)| b - a)
                        .collect();
                }

                sum
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<i32> {
    Some(
        input
            .lines()
            .map(|line| {
                let mut numbers = line
                    .split(' ')
                    .map(|n| n.parse().unwrap())
                    .collect::<Vec<i32>>();

                let mut firsts = vec![];
                while numbers.iter().any(|n| *n != 0) {
                    firsts.push(numbers.first().copied().unwrap());

                    numbers = numbers
                        .iter()
                        .zip(numbers.iter().skip(1))
                        .map(|(a, b)| b - a)
                        .collect();
                }

                let mut sum = 0;
                for n in firsts.into_iter().rev() {
                    sum = n - sum;
                }
                sum
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
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
