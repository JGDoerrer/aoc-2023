use std::collections::HashSet;

advent_of_code::solution!(16);

pub fn part_one(input: &str) -> Option<u32> {
    #[derive(Clone, Copy)]
    enum Tile {
        Empty,
        Mirror1,
        Mirror2,
        HSplitter,
        VSplitter,
    }
    #[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
    enum BeamDir {
        Up,
        Down,
        Left,
        Right,
    }

    let tiles: Vec<Vec<_>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Tile::Empty,
                    '/' => Tile::Mirror1,
                    '\\' => Tile::Mirror2,
                    '-' => Tile::HSplitter,
                    '|' => Tile::VSplitter,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    let mut charged = vec![vec![false; tiles[0].len()]; tiles.len()];
    let mut beams = HashSet::from([(-1, 0, BeamDir::Right)]);
    let mut old_beams: HashSet<(i32, i32, BeamDir)> = HashSet::new();

    while !beams.is_empty() {
        let mut new_beams = HashSet::new();
        for (x, y, dir) in &beams {
            let (x, y, dir) = (*x, *y, *dir);
            let (nx, ny) = match dir {
                BeamDir::Up => (x, y - 1),
                BeamDir::Down => (x, y + 1),
                BeamDir::Left => (x - 1, y),
                BeamDir::Right => (x + 1, y),
            };

            if nx < 0 || nx >= tiles[0].len() as i32 || ny < 0 || ny >= tiles.len() as i32 {
                continue;
            }
            charged[ny as usize][nx as usize] = true;

            let tile = &tiles[ny as usize][nx as usize];

            let mut new_dir = dir;
            match *tile {
                Tile::Empty => {}
                Tile::Mirror1 => {
                    new_dir = match dir {
                        BeamDir::Up => BeamDir::Right,
                        BeamDir::Down => BeamDir::Left,
                        BeamDir::Left => BeamDir::Down,
                        BeamDir::Right => BeamDir::Up,
                    }
                }
                Tile::Mirror2 => {
                    new_dir = match dir {
                        BeamDir::Up => BeamDir::Left,
                        BeamDir::Down => BeamDir::Right,
                        BeamDir::Left => BeamDir::Up,
                        BeamDir::Right => BeamDir::Down,
                    }
                }
                Tile::HSplitter => match dir {
                    BeamDir::Left | BeamDir::Right => {}
                    BeamDir::Up | BeamDir::Down => {
                        if !old_beams.contains(&(nx, ny, BeamDir::Left)) {
                            new_beams.insert((nx, ny, BeamDir::Left));
                        }
                        new_dir = BeamDir::Right;
                    }
                },
                Tile::VSplitter => match dir {
                    BeamDir::Up | BeamDir::Down => {}
                    BeamDir::Left | BeamDir::Right => {
                        if !old_beams.contains(&(nx, ny, BeamDir::Up)) {
                            new_beams.insert((nx, ny, BeamDir::Up));
                        }
                        new_dir = BeamDir::Down;
                    }
                },
            }

            if !old_beams.contains(&(nx, ny, new_dir)) {
                new_beams.insert((nx, ny, new_dir));
            }
        }

        let old_old_beams = old_beams.clone();
        old_beams.extend(new_beams.iter());
        if old_old_beams == old_beams {
            break;
        }

        beams = new_beams;
    }

    Some(
        charged
            .into_iter()
            .map(|line| line.into_iter().filter(|c| *c).count() as u32)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    #[derive(Clone, Copy)]
    enum Tile {
        Empty,
        Mirror1,
        Mirror2,
        HSplitter,
        VSplitter,
    }
    #[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
    enum BeamDir {
        Up,
        Down,
        Left,
        Right,
    }

    let tiles: Vec<Vec<_>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Tile::Empty,
                    '/' => Tile::Mirror1,
                    '\\' => Tile::Mirror2,
                    '-' => Tile::HSplitter,
                    '|' => Tile::VSplitter,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    let start_beams: Vec<_> = (0..tiles[0].len())
        .map(|x| (x as i32, -1i32, BeamDir::Down))
        .chain((0..tiles[0].len()).map(|x| (x as i32, tiles.len() as i32, BeamDir::Up)))
        .chain((0..tiles.len()).map(|y| (-1, y as i32, BeamDir::Right)))
        .chain((0..tiles.len()).map(|y| (tiles[0].len() as i32, y as i32, BeamDir::Left)))
        .collect();
    let mut max_charched = 0;

    for (x, y, dir) in start_beams {
        let mut charged = vec![vec![false; tiles[0].len()]; tiles.len()];
        let mut beams = HashSet::from([(x, y, dir)]);
        let mut old_beams: HashSet<(i32, i32, BeamDir)> = HashSet::new();

        while !beams.is_empty() {
            let mut new_beams = HashSet::new();
            for (x, y, dir) in &beams {
                let (x, y, dir) = (*x, *y, *dir);
                let (nx, ny) = match dir {
                    BeamDir::Up => (x, y - 1),
                    BeamDir::Down => (x, y + 1),
                    BeamDir::Left => (x - 1, y),
                    BeamDir::Right => (x + 1, y),
                };

                if nx < 0 || nx >= tiles[0].len() as i32 || ny < 0 || ny >= tiles.len() as i32 {
                    continue;
                }
                charged[ny as usize][nx as usize] = true;

                let tile = &tiles[ny as usize][nx as usize];

                let mut new_dir = dir;
                match *tile {
                    Tile::Empty => {}
                    Tile::Mirror1 => {
                        new_dir = match dir {
                            BeamDir::Up => BeamDir::Right,
                            BeamDir::Down => BeamDir::Left,
                            BeamDir::Left => BeamDir::Down,
                            BeamDir::Right => BeamDir::Up,
                        }
                    }
                    Tile::Mirror2 => {
                        new_dir = match dir {
                            BeamDir::Up => BeamDir::Left,
                            BeamDir::Down => BeamDir::Right,
                            BeamDir::Left => BeamDir::Up,
                            BeamDir::Right => BeamDir::Down,
                        }
                    }
                    Tile::HSplitter => match dir {
                        BeamDir::Left | BeamDir::Right => {}
                        BeamDir::Up | BeamDir::Down => {
                            if !old_beams.contains(&(nx, ny, BeamDir::Left)) {
                                new_beams.insert((nx, ny, BeamDir::Left));
                            }
                            new_dir = BeamDir::Right;
                        }
                    },
                    Tile::VSplitter => match dir {
                        BeamDir::Up | BeamDir::Down => {}
                        BeamDir::Left | BeamDir::Right => {
                            if !old_beams.contains(&(nx, ny, BeamDir::Up)) {
                                new_beams.insert((nx, ny, BeamDir::Up));
                            }
                            new_dir = BeamDir::Down;
                        }
                    },
                }

                if !old_beams.contains(&(nx, ny, new_dir)) {
                    new_beams.insert((nx, ny, new_dir));
                }
            }

            let old_old_beams = old_beams.clone();
            old_beams.extend(new_beams.iter());
            if old_old_beams == old_beams {
                break;
            }

            beams = new_beams;
        }

        max_charched = max_charched.max(
            charged
                .into_iter()
                .map(|line| line.into_iter().filter(|c| *c).count() as u32)
                .sum(),
        );
    }

    Some(max_charched)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(51));
    }
}
