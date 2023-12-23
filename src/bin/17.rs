use std::{cmp::Reverse, collections::BinaryHeap};

advent_of_code::solution!(17);

pub fn part_one(input: &str) -> Option<u32> {
    let loss: Vec<Vec<_>> = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    #[derive(PartialEq, Eq, Debug, PartialOrd, Ord, Clone)]
    enum Dir {
        Left,
        Right,
        Up,
        Down,
    }

    #[derive(PartialEq, Eq, Debug, Ord, Clone)]
    struct State {
        loss: u32,
        x: i32,
        y: i32,
        straight: u32,
        last: Dir,
        max_x: i32,
        max_y: i32,
    }

    impl PartialOrd for State {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            let a = self.loss + self.x.abs_diff(self.max_x) + self.y.abs_diff(self.max_y);
            let b = other.loss + other.x.abs_diff(other.max_x) + other.y.abs_diff(other.max_y);

            Some(a.cmp(&b))
        }
    }

    let mut map = vec![vec![[u32::MAX; 12]; loss[0].len()]; loss.len()];

    let mut paths: BinaryHeap<Reverse<State>> = BinaryHeap::from([
        Reverse(State {
            loss: 0,
            x: 0,
            y: 0,
            straight: 0,
            last: Dir::Right,
            max_x: loss[0].len() as i32,
            max_y: loss.len() as i32,
        }),
        Reverse(State {
            loss: 0,
            x: 0,
            y: 0,
            straight: 0,
            last: Dir::Down,
            max_x: loss[0].len() as i32,
            max_y: loss.len() as i32,
        }),
    ]);

    let min = loop {
        let state = paths.pop().unwrap().0;
        if state.x == loss[0].len() as i32 - 1 && state.y == loss.len() as i32 - 1 {
            break state.loss;
        }

        let mut next_dirs = vec![Dir::Left, Dir::Right, Dir::Up, Dir::Down];
        next_dirs.retain(|dir| {
            *dir != match state.last {
                Dir::Left => Dir::Right,
                Dir::Right => Dir::Left,
                Dir::Down => Dir::Up,
                Dir::Up => Dir::Down,
            }
        });
        if state.straight >= 2 {
            next_dirs.retain(|dir| *dir != state.last);
        }

        for dir in next_dirs {
            let (nx, ny) = match dir {
                Dir::Left => (state.x - 1, state.y),
                Dir::Right => (state.x + 1, state.y),
                Dir::Up => (state.x, state.y - 1),
                Dir::Down => (state.x, state.y + 1),
            };

            if nx < 0 || nx >= loss[0].len() as i32 || ny < 0 || ny >= loss.len() as i32 {
                continue;
            }

            let new_loss = state.loss + loss[ny as usize][nx as usize];

            let new_state = State {
                loss: new_loss,
                x: nx,
                y: ny,
                straight: if dir == state.last {
                    state.straight + 1
                } else {
                    0
                },
                last: dir,
                max_x: state.max_x,
                max_y: state.max_y,
            };

            let index = new_state.straight * 4
                + match new_state.last {
                    Dir::Left => 0,
                    Dir::Right => 1,
                    Dir::Up => 2,
                    Dir::Down => 3,
                };
            let min = map[ny as usize][nx as usize][index as usize];

            if new_state.loss >= min {
                continue;
            }

            map[ny as usize][nx as usize][index as usize] = new_state.loss;

            // paths.retain(|s| {
            //     if s.0.x != new_state.x
            //         || s.0.y != new_state.y
            //         || s.0.last != new_state.last
            //         || s.0.straight != new_state.straight
            //     {
            //         true
            //     } else {
            //         let index = s.0.straight * 4
            //             + match s.0.last {
            //                 Dir::Left => 0,
            //                 Dir::Right => 1,
            //                 Dir::Up => 2,
            //                 Dir::Down => 3,
            //             };
            //         let min = map[s.0.y as usize][s.0.x as usize][index as usize];
            //         s.0.loss <= min
            //     }
            // });

            // paths.retain(|s| {
            //     s.0.x != new_state.x
            //         || s.0.y != new_state.y
            //         || s.0.last != new_state.last
            //         || s.0.straight != new_state.straight
            // });

            paths.push(Reverse(new_state.clone()));
        }
    };

    Some(min)
}

pub fn part_two(input: &str) -> Option<u32> {
    let loss: Vec<Vec<_>> = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    #[derive(PartialEq, Eq, Debug, PartialOrd, Ord, Clone)]
    enum Dir {
        Left,
        Right,
        Up,
        Down,
    }

    #[derive(PartialEq, Eq, Debug, Ord, Clone)]
    struct State {
        loss: u32,
        x: i32,
        y: i32,
        straight: u32,
        last: Dir,
        max_x: i32,
        max_y: i32,
        history: Vec<(i32, i32)>,
    }

    impl PartialOrd for State {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            let a = self.loss;
            let b = other.loss;

            Some(a.cmp(&b))
        }
    }

    let mut map = vec![vec![[u32::MAX; 44]; loss[0].len()]; loss.len()];

    let mut paths: BinaryHeap<Reverse<State>> = BinaryHeap::from([
        Reverse(State {
            loss: 0,
            x: 0,
            y: 0,
            straight: 0,
            last: Dir::Right,
            max_x: loss[0].len() as i32,
            max_y: loss.len() as i32,
            history: vec![],
        }),
        Reverse(State {
            loss: 0,
            x: 0,
            y: 0,
            straight: 0,
            last: Dir::Down,
            max_x: loss[0].len() as i32,
            max_y: loss.len() as i32,
            history: vec![],
        }),
    ]);

    let min = loop {
        let state = paths.pop().unwrap().0;
        if state.x == loss[0].len() as i32 - 1 && state.y == loss.len() as i32 - 1 {
            break state.loss;
        }

        let mut next_dirs = vec![Dir::Left, Dir::Right, Dir::Up, Dir::Down];
        next_dirs.retain(|dir| {
            *dir != match state.last {
                Dir::Left => Dir::Right,
                Dir::Right => Dir::Left,
                Dir::Down => Dir::Up,
                Dir::Up => Dir::Down,
            }
        });
        if state.straight <= 3 {
            next_dirs = vec![state.last.clone()];
        } else if state.straight > 9 {
            next_dirs.retain(|dir| *dir != state.last);
        }

        for dir in next_dirs {
            let (nx, ny) = match dir {
                Dir::Left => (state.x - 1, state.y),
                Dir::Right => (state.x + 1, state.y),
                Dir::Up => (state.x, state.y - 1),
                Dir::Down => (state.x, state.y + 1),
            };

            if nx < 0 || nx >= loss[0].len() as i32 || ny < 0 || ny >= loss.len() as i32 {
                continue;
            }

            let new_loss = state.loss + loss[ny as usize][nx as usize];
            let mut history = state.history.clone();
            history.push((state.x, state.y));

            let new_state = State {
                loss: new_loss,
                x: nx,
                y: ny,
                straight: if dir == state.last {
                    state.straight + 1
                } else {
                    1
                },
                last: dir,
                max_x: state.max_x,
                max_y: state.max_y,
                history,
            };

            let index = new_state.straight * 4
                + match new_state.last {
                    Dir::Left => 0,
                    Dir::Right => 1,
                    Dir::Up => 2,
                    Dir::Down => 3,
                };
            let min = map[ny as usize][nx as usize][index as usize];

            if new_state.loss >= min {
                continue;
            }

            map[ny as usize][nx as usize][index as usize] = new_state.loss;

            // paths.retain(|s| {
            //     if s.0.x != new_state.x
            //         || s.0.y != new_state.y
            //         || s.0.last != new_state.last
            //         || s.0.straight != new_state.straight
            //     {
            //         true
            //     } else {
            //         let index = s.0.straight * 4
            //             + match s.0.last {
            //                 Dir::Left => 0,
            //                 Dir::Right => 1,
            //                 Dir::Up => 2,
            //                 Dir::Down => 3,
            //             };
            //         let min = map[s.0.y as usize][s.0.x as usize][index as usize];
            //         s.0.loss <= min
            //     }
            // });

            // paths.retain(|s| {
            //     s.0.x != new_state.x
            //         || s.0.y != new_state.y
            //         || s.0.last != new_state.last
            //         || s.0.straight != new_state.straight
            // });

            paths.push(Reverse(new_state.clone()));
        }
    };

    Some(min)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(102));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));
    }
}
