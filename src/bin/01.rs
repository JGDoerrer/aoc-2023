advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let first: u32 = line
                    .chars()
                    .filter(|c| c.is_digit(10))
                    .map(|c| c.to_string().parse().unwrap())
                    .next()
                    .unwrap();
                let last: u32 = line
                    .chars()
                    .rev()
                    .filter(|c| c.is_digit(10))
                    .map(|c| c.to_string().parse().unwrap())
                    .next()
                    .unwrap();
                first * 10 + last
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
                    .filter(|(_, c)| c.is_digit(10))
                    .map(|(i, c)| (i, c.to_string().parse().unwrap()))
                    .next()
                    .unwrap();
                let (last_index, last): (usize, u32) = line
                    .chars()
                    .rev()
                    .enumerate()
                    .filter(|(_, c)| c.is_digit(10))
                    .map(|(i, c)| (line.len() - i - 1, c.to_string().parse().unwrap()))
                    .next()
                    .unwrap();

                let rev_line: String = line.chars().rev().collect();
                let indices = [
                    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
                ]
                .map(|num| {
                    let rev_num: String = num.chars().rev().collect();

                    let first = line.find(num);
                    let last = line.rmatch_indices(num).next().map(|(i, _)| i);

                    (first, last)
                });

                let min = indices
                    .iter()
                    .enumerate()
                    .filter_map(|(i, (f, _))| f.map(|f| (i, f)))
                    .min_by_key(|(i, f)| *f);

                let max = indices
                    .iter()
                    .enumerate()
                    .filter_map(|(i, (f, _))| f.map(|f| (i, f)))
                    .max_by_key(|(i, f)| *f);

                let min = min.map_or(
                    first,
                    |(i, f)| if f < first_index { i as u32 + 1 } else { first },
                );
                let max = max.map_or(
                    last,
                    |(i, f)| if f > last_index { i as u32 + 1 } else { last },
                );

                dbg!(first, last, min, max, line);
                dbg!(min * 10 + max)
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
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(281 - 83));
    }
}
