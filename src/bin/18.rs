advent_of_code::solution!(18);

pub fn part_one(input: &str) -> Option<u32> {
    enum Dir {
        Up,
        Down,
        Left,
        Right,
    }

    let instr = input.lines().map(|line| {
        let (dir, len) = line.split_at(1);
        let dir = match dir.chars().next().unwrap() {
            'R' => Dir::Right,
            'D' => Dir::Down,
            'U' => Dir::Up,
            'L' => Dir::Left,
            _ => unreachable!(),
        };
        let len = len[1..].split_once(' ').unwrap().0.parse().unwrap();
        (dir, len)
    });

    let mut map = vec![vec![false; 1000]; 1000];

    let mut pos = (500, 500);
    for (dir, len) in instr {
        match dir {
            Dir::Down => {
                for dy in 0..len {
                    map[pos.0][pos.1 + dy] = true;
                }
                pos.1 += len;
            }
            Dir::Up => {
                for dy in 0..len {
                    map[pos.0][pos.1 - dy] = true;
                }
                pos.1 -= len;
            }
            Dir::Left => {
                for dx in 0..len {
                    map[pos.0 - dx][pos.1] = true;
                }
                pos.0 -= len;
            }
            Dir::Right => {
                for dx in 0..len {
                    map[pos.0 + dx][pos.1] = true;
                }
                pos.0 += len;
            }
        }
    }

    let mut queue = vec![(501, 501)];
    while let Some((x, y)) = queue.pop() {
        if map[x][y] {
            continue;
        }

        map[x][y] = true;
        queue.push((x.saturating_sub(1), y));
        queue.push((x + 1, y));
        queue.push((x, y.saturating_sub(1)));
        queue.push((x, y + 1));
    }

    Some(
        map.into_iter()
            .map(|line| line.into_iter().filter(|x| *x).count() as u32)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    #[derive(Clone)]
    enum Dir {
        Up,
        Down,
        Left,
        Right,
    }

    let instr = input.lines().map(|line| {
        let len = &line[2..];
        let dir = match &len.split_once(' ').unwrap().1[7..].chars().next().unwrap() {
            '0' => Dir::Right,
            '1' => Dir::Down,
            '3' => Dir::Up,
            '2' => Dir::Left,
            _ => unreachable!(),
        };
        let len = i64::from_str_radix(&len.split_once(' ').unwrap().1[2..7], 16).unwrap();
        (dir, len)
    });

    let mut area = 0;
    let mut pos = (0, 0);
    for (dir, len) in instr {
        let next_pos = match dir {
            Dir::Up => (pos.0, pos.1 + len),
            Dir::Down => (pos.0, pos.1 - len),
            Dir::Left => (pos.0 + len, pos.1),
            Dir::Right => (pos.0 - len, pos.1),
        };

        area += ((pos.0 - next_pos.0) * (pos.1 + next_pos.1))
            + pos.0.abs_diff(next_pos.0) as i64
            + pos.1.abs_diff(next_pos.1) as i64;

        pos = next_pos;
    }

    Some(1 + area as u64 / 2)
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
