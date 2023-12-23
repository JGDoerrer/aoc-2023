advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u32> {
    let space: Vec<Vec<_>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => false,
                    '#' => true,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    let empty_rows: Vec<_> = (0..space.len())
        .filter(|i| space[*i].iter().all(|b| !b))
        .collect();

    let empty_cols: Vec<_> = (0..space[0].len())
        .filter(|i| space.iter().all(|row| !row[*i]))
        .collect();

    let galaxies: Vec<_> = space
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter_map(|(j, g)| if *g { Some((i, j)) } else { None })
                .collect::<Vec<_>>()
        })
        .map(|(y, x)| {
            (
                y + empty_rows.iter().filter(|i| **i < y).count(),
                x + empty_cols.iter().filter(|i| **i < x).count(),
            )
        })
        .collect();

    let mut sum = 0;
    for i in 0..galaxies.len() {
        let (y, x) = galaxies[i];

        for (y2, x2) in galaxies.iter().skip(i + 1) {
            sum += y.abs_diff(*y2) + x.abs_diff(*x2);
        }
    }

    Some(sum as u32)
}

pub fn part_two(input: &str) -> Option<u64> {
    let space: Vec<Vec<_>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => false,
                    '#' => true,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    let empty_rows: Vec<_> = (0..space.len())
        .filter(|i| space[*i].iter().all(|b| !b))
        .collect();

    let empty_cols: Vec<_> = (0..space[0].len())
        .filter(|i| space.iter().all(|row| !row[*i]))
        .collect();

    let galaxies: Vec<_> = space
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter_map(|(j, g)| if *g { Some((i, j)) } else { None })
                .collect::<Vec<_>>()
        })
        .map(|(y, x)| {
            (
                y + empty_rows.iter().filter(|i| **i < y).count() * 999999,
                x + empty_cols.iter().filter(|i| **i < x).count() * 999999,
            )
        })
        .collect();

    let mut sum = 0;
    for i in 0..galaxies.len() {
        let (y, x) = galaxies[i];

        for (y2, x2) in galaxies.iter().skip(i + 1) {
            sum += y.abs_diff(*y2) + x.abs_diff(*x2);
        }
    }

    Some(sum as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(82000210));
    }
}
