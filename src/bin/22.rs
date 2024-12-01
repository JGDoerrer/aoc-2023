use std::array;

advent_of_code::solution!(22);

pub fn part_one(input: &str) -> Option<u32> {
    let mut bricks: Vec<_> = input
        .lines()
        .map(|line| {
            let (a, b) = line.split_once('~').unwrap();
            let a: [usize; 3] = a
                .splitn(3, ',')
                .map(|p| p.parse().unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
            let b: [usize; 3] = b
                .splitn(3, ',')
                .map(|p| p.parse().unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
            (a, b)
        })
        .collect();

    bricks.sort_by_key(|(a, b)| a[2].min(b[2]));

    let mut ground: [[Option<(usize, usize)>; 10]; 10] =
        array::from_fn(|_| array::from_fn(|_| None));

    let mut dependencies = Vec::new();

    for (i, (a, b)) in bricks.iter().enumerate() {
        if a[0] != b[0] {
            let y = a[1];

            let mut max_z = 0;

            let min_x = a[0].min(b[0]);
            let max_x = a[0].max(b[0]);

            let mut deps = Vec::new();

            for x in min_x..=max_x {
                if let Some((index, z)) = ground[x][y] {
                    if z > max_z {
                        deps = vec![index];
                        max_z = z;
                    } else if z == max_z {
                        deps.push(index);
                    }
                }
            }

            for x in min_x..=max_x {
                ground[x][y] = Some((i, max_z + 1))
            }

            for j in deps {
                dependencies.push((j, i));
            }
        } else if a[1] != b[1] {
            let x = a[0];

            let mut max_z = 0;

            let min_y = a[1].min(b[1]);
            let max_y = a[1].max(b[1]);

            let mut deps = Vec::new();

            for y in min_y..=max_y {
                if let Some((index, z)) = ground[x][y] {
                    if z > max_z {
                        deps = vec![index];
                        max_z = z;
                    } else if z == max_z {
                        deps.push(index);
                    }
                }
            }

            for y in min_y..=max_y {
                ground[x][y] = Some((i, max_z + 1))
            }

            for j in deps {
                dependencies.push((j, i));
            }
        } else {
            let (x, y) = (a[0], a[1]);

            let z = ground[x][y];

            if let Some((index, z)) = z {
                ground[x][y] = Some((i, z + a[2].abs_diff(b[2]) + 1));
                dependencies.push((index, i));
            } else {
                ground[x][y] = Some((i, a[2].abs_diff(b[2]) + 1));
            }
        }
    }

    let mut count = 0;

    for i in 0..bricks.len() {
        let supporting: Vec<_> = dependencies
            .iter()
            .filter(|(j, _)| *j == i)
            .map(|(_, j)| *j)
            .collect();

        let mut removable = true;
        for k in supporting {
            let supported = dependencies
                .iter()
                .filter(|(l, j)| *j == k && *l != i)
                .count();
            if supported == 0 {
                removable = false;
                break;
            }
        }

        if removable {
            count += 1;
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut bricks: Vec<_> = input
        .lines()
        .map(|line| {
            let (a, b) = line.split_once('~').unwrap();
            let a: [usize; 3] = a
                .splitn(3, ',')
                .map(|p| p.parse().unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
            let b: [usize; 3] = b
                .splitn(3, ',')
                .map(|p| p.parse().unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
            (a, b)
        })
        .collect();

    bricks.sort_by_key(|(a, b)| a[2].min(b[2]));

    let mut ground: [[Option<(usize, usize)>; 10]; 10] =
        array::from_fn(|_| array::from_fn(|_| None));

    let mut dependencies = Vec::new();

    for (i, (a, b)) in bricks.iter().enumerate() {
        if a[0] != b[0] {
            let y = a[1];

            let mut max_z = 0;

            let min_x = a[0].min(b[0]);
            let max_x = a[0].max(b[0]);

            let mut deps = Vec::new();

            for x in min_x..=max_x {
                if let Some((index, z)) = ground[x][y] {
                    if z > max_z {
                        deps = vec![index];
                        max_z = z;
                    } else if z == max_z {
                        deps.push(index);
                    }
                }
            }

            for x in min_x..=max_x {
                ground[x][y] = Some((i, max_z + 1))
            }

            for j in deps {
                dependencies.push((j, i));
            }
        } else if a[1] != b[1] {
            let x = a[0];

            let mut max_z = 0;

            let min_y = a[1].min(b[1]);
            let max_y = a[1].max(b[1]);

            let mut deps = Vec::new();

            for y in min_y..=max_y {
                if let Some((index, z)) = ground[x][y] {
                    if z > max_z {
                        deps = vec![index];
                        max_z = z;
                    } else if z == max_z {
                        deps.push(index);
                    }
                }
            }

            for y in min_y..=max_y {
                ground[x][y] = Some((i, max_z + 1))
            }

            for j in deps {
                dependencies.push((j, i));
            }
        } else {
            let (x, y) = (a[0], a[1]);

            let z = ground[x][y];

            if let Some((index, z)) = z {
                ground[x][y] = Some((i, z + a[2].abs_diff(b[2]) + 1));
                dependencies.push((index, i));
            } else {
                ground[x][y] = Some((i, a[2].abs_diff(b[2]) + 1));
            }
        }
    }

    let mut count = 0;

    for i in 0..bricks.len() {
        let mut falling = vec![i];
        let mut stack = vec![i];

        while let Some(i) = stack.pop() {
            let supporting: Vec<_> = dependencies
                .iter()
                .filter(|(j, _)| *j == i)
                .map(|(_, j)| *j)
                .collect();

            for k in supporting {
                let supported = dependencies
                    .iter()
                    .filter(|(l, j)| *j == k && !falling.contains(l))
                    .count();

                if supported == 0 && !falling.contains(&k) {
                    falling.push(k);
                    stack.push(k);
                }
            }
        }
        falling.remove(0);

        count += falling.len() as u32;
    }

    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }
}
