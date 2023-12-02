advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let first = line
                    .chars()
                    .filter(|c| c.is_ascii_digit())
                    .map(|c| c.to_digit(10).unwrap())
                    .next()
                    .unwrap();
                let second = line
                    .chars()
                    .rev()
                    .filter(|c| c.is_ascii_digit())
                    .map(|c| c.to_digit(10).unwrap())
                    .next()
                    .unwrap();

                first * 10 + second
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let (first_index, first): (usize, u32) = line
                    .chars()
                    .enumerate()
                    .filter(|(_, c)| c.is_ascii_digit())
                    .map(|(i, c)| (i, c.to_digit(10).unwrap()))
                    .next()
                    .unwrap();
                let (last_index, last): (usize, u32) = line
                    .chars()
                    .rev()
                    .enumerate()
                    .filter(|(_, c)| c.is_ascii_digit())
                    .map(|(i, c)| (line.len() - i - 1, c.to_digit(10).unwrap()))
                    .next()
                    .unwrap();

                let indices = [
                    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
                ]
                .map(|num| (line.find(num), line.rfind(num)));

                let first = indices
                    .iter()
                    .enumerate()
                    .filter_map(|(i, (f, _))| f.map(|f| (i + 1, f)))
                    .min_by_key(|(_, f)| *f)
                    .map_or(
                        first,
                        |(i, f)| if f < first_index { i as u32 } else { first },
                    );

                let second = indices
                    .iter()
                    .enumerate()
                    .filter_map(|(i, (_, f))| f.map(|f| (i + 1, f)))
                    .max_by_key(|(_, f)| *f)
                    .map_or(last, |(i, f)| if f > last_index { i as u32 } else { last });

                first * 10 + second
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
        assert_eq!(result, Some(209));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(281 - 83));
    }
}
