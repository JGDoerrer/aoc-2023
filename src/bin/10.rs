advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<u32> {
    #[derive(PartialEq, Eq, Clone, Copy, Debug)]
    enum Pipe {
        NS,
        EW,
        NE,
        NW,
        SE,
        SW,
        Start,
    }

    let pipes = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => None,
                    '-' => Some(Pipe::EW),
                    '|' => Some(Pipe::NS),
                    'F' => Some(Pipe::SE),
                    'J' => Some(Pipe::NW),
                    'L' => Some(Pipe::NE),
                    '7' => Some(Pipe::SW),
                    'S' => Some(Pipe::Start),
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let start_pos = pipes
        .iter()
        .enumerate()
        .map(|(i, line)| (i, line.iter().position(|p| *p == Some(Pipe::Start))))
        .filter(|(_, l)| l.is_some())
        .map(|(i, j)| (i, j.unwrap()))
        .next()
        .unwrap();

    let starts: [_; 2] = [
        (start_pos.0 as isize + 1, start_pos.1 as isize),
        (start_pos.0 as isize - 1, start_pos.1 as isize),
        (start_pos.0 as isize, start_pos.1 as isize + 1),
        (start_pos.0 as isize, start_pos.1 as isize - 1),
    ]
    .iter()
    .filter(|(y, x)| {
        *y >= 0 && *y < pipes.len() as isize && *x >= 0 && *x < pipes[0].len() as isize
    })
    .filter(|(y, x)| {
        let (dy, dx) = (start_pos.0 as isize - y, start_pos.1 as isize - x);
        let pipe = pipes[*y as usize][*x as usize];
        match (dy, dx) {
            (1, 0) => match pipe {
                Some(Pipe::NS) | Some(Pipe::SE) | Some(Pipe::SW) => true,
                _ => false,
            },
            (-1, 0) => match pipe {
                Some(Pipe::NS) | Some(Pipe::NE) | Some(Pipe::NW) => true,
                _ => false,
            },
            (0, 1) => match pipe {
                Some(Pipe::EW) | Some(Pipe::NE) | Some(Pipe::SE) => true,
                _ => false,
            },
            (0, -1) => match pipe {
                Some(Pipe::EW) | Some(Pipe::NW) | Some(Pipe::SW) => true,
                _ => false,
            },
            _ => unreachable!(),
        }
    })
    .map(|(y, x)| (*y as usize, *x as usize))
    .collect::<Vec<_>>()
    .try_into()
    .unwrap();

    let mut distances = vec![vec![None; pipes[0].len()]; pipes.len()];
    distances[start_pos.0][start_pos.1] = Some(0);

    let mut last = start_pos;
    let mut pos = starts[0];
    let mut distance = 1;
    loop {
        distances[pos.0][pos.1] = Some(distance);
        let (dy, dx) = (
            pos.0 as isize - last.0 as isize,
            pos.1 as isize - last.1 as isize,
        );
        let pipe = pipes[pos.0][pos.1].unwrap();
        let (ndy, ndx) = match pipe {
            Pipe::Start => break,
            Pipe::NS => (dy, dx),
            Pipe::EW => (dy, dx),
            Pipe::NE => {
                if (dy, dx) == (1, 0) {
                    (0, 1)
                } else {
                    (-1, 0)
                }
            }
            Pipe::NW => {
                if (dy, dx) == (1, 0) {
                    (0, -1)
                } else {
                    (-1, 0)
                }
            }
            Pipe::SE => {
                if (dy, dx) == (-1, 0) {
                    (0, 1)
                } else {
                    (1, 0)
                }
            }
            Pipe::SW => {
                if (dy, dx) == (-1, 0) {
                    (0, -1)
                } else {
                    (1, 0)
                }
            }
        };
        let next = (
            (pos.0 as isize + ndy) as usize,
            (pos.1 as isize + ndx) as usize,
        );
        (last, pos) = (pos, next);
        distance += 1;
    }

    Some(distance / 2)
}

pub fn part_two(input: &str) -> Option<u32> {
    #[derive(PartialEq, Eq, Clone, Copy, Debug)]
    enum Pipe {
        NS,
        EW,
        NE,
        NW,
        SE,
        SW,
        Start,
    }

    let mut pipes = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => None,
                    '-' => Some(Pipe::EW),
                    '|' => Some(Pipe::NS),
                    'F' => Some(Pipe::SE),
                    'J' => Some(Pipe::NW),
                    'L' => Some(Pipe::NE),
                    '7' => Some(Pipe::SW),
                    'S' => Some(Pipe::Start),
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let start_pos = pipes
        .iter()
        .enumerate()
        .map(|(i, line)| (i, line.iter().position(|p| *p == Some(Pipe::Start))))
        .filter(|(_, l)| l.is_some())
        .map(|(i, j)| (i, j.unwrap()))
        .next()
        .unwrap();

    let starts: [_; 2] = [(1isize, 0isize), (0 - 1, 0), (0, 0 + 1), (0, 0 - 1)]
        .into_iter()
        .filter(|(y, x)| {
            (*y + start_pos.0 as isize) >= 0
                && (*y + start_pos.0 as isize) < pipes.len() as isize
                && (*x + start_pos.1 as isize) >= 0
                && (*x + start_pos.1 as isize) < pipes[0].len() as isize
        })
        .filter(|(dy, dx)| {
            let pipe =
                pipes[(*dy + start_pos.0 as isize) as usize][(*dx + start_pos.1 as isize) as usize];
            (pipe, (dy, dx));
            match (dy, dx) {
                (-1, 0) => match pipe {
                    Some(Pipe::NS) | Some(Pipe::SE) | Some(Pipe::SW) => true,
                    _ => false,
                },
                (1, 0) => match pipe {
                    Some(Pipe::NS) | Some(Pipe::NE) | Some(Pipe::NW) => true,
                    _ => false,
                },
                (0, -1) => match pipe {
                    Some(Pipe::EW) | Some(Pipe::NE) | Some(Pipe::SE) => true,
                    _ => false,
                },
                (0, 1) => match pipe {
                    Some(Pipe::EW) | Some(Pipe::NW) | Some(Pipe::SW) => true,
                    _ => false,
                },
                _ => unreachable!(),
            }
        })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    pipes[start_pos.0][start_pos.1] = if starts.contains(&(1, 0)) && starts.contains(&(-1, 0)) {
        Some(Pipe::NS)
    } else if starts.contains(&(0, 1)) && starts.contains(&(0, -1)) {
        Some(Pipe::EW)
    } else if starts.contains(&(1, 0)) && starts.contains(&(0, 1)) {
        Some(Pipe::SE)
    } else if starts.contains(&(1, 0)) && starts.contains(&(0, -1)) {
        Some(Pipe::SW)
    } else if starts.contains(&(-1, 0)) && starts.contains(&(0, -1)) {
        Some(Pipe::NW)
    } else if starts.contains(&(-1, 0)) && starts.contains(&(0, 1)) {
        Some(Pipe::NE)
    } else {
        unreachable!()
    };

    let mut visited = vec![vec![false; pipes[0].len()]; pipes.len()];

    let mut last = start_pos;
    let mut pos = (
        (start_pos.0 as isize + starts[0].0) as usize,
        (start_pos.1 as isize + starts[0].1) as usize,
    );
    loop {
        if visited[pos.0][pos.1] {
            break;
        }

        visited[pos.0][pos.1] = true;
        let (dy, dx) = (
            pos.0 as isize - last.0 as isize,
            pos.1 as isize - last.1 as isize,
        );
        let pipe = pipes[pos.0][pos.1].unwrap();
        let (ndy, ndx) = match pipe {
            Pipe::Start => break,
            Pipe::NS => (dy, dx),
            Pipe::EW => (dy, dx),
            Pipe::NE => {
                if (dy, dx) == (1, 0) {
                    (0, 1)
                } else {
                    (-1, 0)
                }
            }
            Pipe::NW => {
                if (dy, dx) == (1, 0) {
                    (0, -1)
                } else {
                    (-1, 0)
                }
            }
            Pipe::SE => {
                if (dy, dx) == (-1, 0) {
                    (0, 1)
                } else {
                    (1, 0)
                }
            }
            Pipe::SW => {
                if (dy, dx) == (-1, 0) {
                    (0, -1)
                } else {
                    (1, 0)
                }
            }
        };
        let next = (
            (pos.0 as isize + ndy) as usize,
            (pos.1 as isize + ndx) as usize,
        );
        (last, pos) = (pos, next);
    }

    let mut count = 0;

    for i in 0..visited.len() {
        let mut inside = false;
        let mut crossed_north = false;
        let mut crossed_south = false;
        for j in 0..visited[0].len() {
            let pipe = pipes[i][j];
            let visited = visited[i][j];

            match pipe {
                Some(Pipe::NS) if visited => {
                    inside = !inside;
                }
                Some(Pipe::NE) | Some(Pipe::NW) if visited => {
                    if crossed_north {
                        crossed_north = false;
                    } else {
                        crossed_north = true;
                    }
                }
                Some(Pipe::SE) | Some(Pipe::SW) if visited => {
                    if crossed_south {
                        crossed_south = false;
                    } else {
                        crossed_south = true;
                    }
                }
                Some(Pipe::EW) if visited => {}
                _ => {
                    if inside {
                        count += 1;
                    } else {
                    }
                }
            }
            if crossed_north && crossed_south {
                inside = !inside;
                crossed_north = false;
                crossed_south = false
            }
        }
    }

    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(80));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10));
    }
}
