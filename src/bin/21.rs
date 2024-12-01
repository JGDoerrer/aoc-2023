advent_of_code::solution!(21);

#[derive(Debug)]
enum Tile {
    Start,
    Rock,
    Plot,
}

pub fn part_one(input: &str) -> Option<u32> {
    let map: Vec<Vec<_>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Tile::Plot,
                    '#' => Tile::Rock,
                    'S' => Tile::Start,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    let mut reachable = vec![vec![false; map[0].len()]; map.len()];

    let start = map
        .iter()
        .enumerate()
        .find_map(|(j, row)| {
            row.iter()
                .enumerate()
                .find_map(|(i, t)| matches!(t, Tile::Start).then_some(i))
                .map(|i| (j, i))
        })
        .unwrap();

    reachable[start.0][start.1] = true;

    for _ in 0..64 {
        let mut new_reachable = vec![vec![false; map[0].len()]; map.len()];

        for i in 0..map.len() {
            for j in 0..map[0].len() {
                if matches!(map[i][j], Tile::Rock) {
                    continue;
                }

                if i > 0 && reachable[i - 1][j]
                    || i < map.len() - 1 && reachable[i + 1][j]
                    || j > 0 && reachable[i][j - 1]
                    || j < map[0].len() - 1 && reachable[i][j + 1]
                {
                    new_reachable[i][j] = true;
                }
            }
        }

        reachable = new_reachable;
    }

    Some(
        reachable
            .into_iter()
            .map(|row| row.into_iter().filter(|r| *r).count() as u32)
            .sum::<u32>(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let map: Vec<Vec<_>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Tile::Plot,
                    '#' => Tile::Rock,
                    'S' => Tile::Start,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    let start = map
        .iter()
        .enumerate()
        .find_map(|(j, row)| {
            row.iter()
                .enumerate()
                .find_map(|(i, t)| matches!(t, Tile::Start).then_some(i))
                .map(|i| (j, i))
        })
        .unwrap();

    let mut reachable = vec![vec![false; map[0].len() * 5]; map.len() * 5];
    reachable[start.0 + map.len() * 2][start.1 + map[0].len() * 2] = true;

    let mut values = [0; 4];

    for k in 0..65 + 131 * 2 {
        let mut new_reachable = vec![vec![false; map[0].len() * 5]; map.len() * 5];

        for i in 0..map.len() * 5 {
            for j in 0..map[0].len() * 5 {
                if matches!(map[i % map.len()][j % map[0].len()], Tile::Rock) {
                    continue;
                }

                if i > 0 && reachable[i - 1][j]
                    || i < map.len() * 5 - 1 && reachable[i + 1][j]
                    || j > 0 && reachable[i][j - 1]
                    || j < map[0].len() * 5 - 1 && reachable[i][j + 1]
                {
                    new_reachable[i][j] = true;
                }
            }
        }

        reachable = new_reachable;

        if k == 64 {
            values[(k - 64) / 131] = reachable
                .iter()
                .map(|row| row.into_iter().filter(|r| **r).count() as u64)
                .sum::<u64>();
        }
        if k == 64 + 131 {
            values[(k - 64) / 131] = reachable
                .iter()
                .map(|row| row.into_iter().filter(|r| **r).count() as u64)
                .sum::<u64>();
        }
        if k == 64 + 131 * 2 {
            values[(k - 64) / 131] = reachable
                .iter()
                .map(|row| row.into_iter().filter(|r| **r).count() as u64)
                .sum::<u64>();
        }
    }

    let repeats = (26501365 - 65) / 131;
    let delta = (values[2] - values[1]) - (values[1] - values[0]);
    let count =
        values[0] + (values[1] - values[0]) * repeats + (delta * repeats * (repeats - 1)) / 2;

    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(42));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
