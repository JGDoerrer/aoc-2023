advent_of_code::solution!(18);

pub fn part_one(input: &str) -> Option<u32> {
    let mut pos = (0, 0);
    Some(
        1 + input
            .lines()
            .map(|line| {
                let (dir, len) = line.split_at(1);
                let len: i32 = len[1..].split_once(' ').unwrap().0.parse().unwrap();
                let next_pos = match dir.chars().next().unwrap() {
                    'R' => (pos.0 - len, pos.1),
                    'D' => (pos.0, pos.1 - len),
                    'U' => (pos.0, pos.1 + len),
                    'L' => (pos.0 + len, pos.1),
                    _ => unreachable!(),
                };

                let area = ((pos.0 - next_pos.0) * (pos.1 + next_pos.1))
                    + pos.0.abs_diff(next_pos.0) as i32
                    + pos.1.abs_diff(next_pos.1) as i32;

                pos = next_pos;
                area
            })
            .sum::<i32>() as u32
            / 2,
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut pos = (0, 0);
    Some(
        1 + input
            .lines()
            .map(|line| {
                let len =
                    i64::from_str_radix(&line[2..].split_once(' ').unwrap().1[2..7], 16).unwrap();
                let next_pos = match &line[2..].split_once(' ').unwrap().1[7..]
                    .chars()
                    .next()
                    .unwrap()
                {
                    '0' => (pos.0 - len, pos.1),
                    '1' => (pos.0, pos.1 - len),
                    '3' => (pos.0, pos.1 + len),
                    '2' => (pos.0 + len, pos.1),
                    _ => unreachable!(),
                };

                let area = ((pos.0 - next_pos.0) * (pos.1 + next_pos.1))
                    + pos.0.abs_diff(next_pos.0) as i64
                    + pos.1.abs_diff(next_pos.1) as i64;

                pos = next_pos;
                area
            })
            .sum::<i64>() as u64
            / 2,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(62));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(952408144115));
    }
}
