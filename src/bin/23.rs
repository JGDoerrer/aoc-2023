advent_of_code::solution!(23);

pub fn part_one(input: &str) -> Option<u32> {
    let map: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();
    let (w, h) = (map.len(), map[0].len());

    let start = (1, 0);
    let end = (w - 2, h - 1);

    let mut stack = vec![(start, start, 0)];
    let mut max_len = 0;

    while let Some(((x, y), last, len)) = stack.pop() {
        if (x, y) == end {
            max_len = max_len.max(len);
        }
        match map[y][x] {
            '.' => {
                if y > 0
                    && (x, y - 1) != last
                    && matches!(map[y - 1][x], '.' | '^' | '>' | '<' | 'v')
                {
                    stack.push(((x, y - 1), (x, y), len + 1));
                }
                if y < map.len() - 1
                    && (x, y + 1) != last
                    && matches!(map[y + 1][x], '.' | '^' | '>' | '<' | 'v')
                {
                    stack.push(((x, y + 1), (x, y), len + 1));
                }
                if (x - 1, y) != last && matches!(map[y][x - 1], '.' | '^' | '>' | '<' | 'v') {
                    stack.push(((x - 1, y), (x, y), len + 1));
                }
                if (x + 1, y) != last && matches!(map[y][x + 1], '.' | '^' | '>' | '<' | 'v') {
                    stack.push(((x + 1, y), (x, y), len + 1));
                }
            }
            '^' => {
                if y > 0
                    && (x, y - 1) != last
                    && matches!(map[y - 1][x], '.' | '^' | '>' | '<' | 'v')
                {
                    stack.push(((x, y - 1), (x, y), len + 1));
                }
            }
            'v' => {
                if y < map.len() - 1
                    && (x, y + 1) != last
                    && matches!(map[y + 1][x], '.' | '^' | '>' | '<' | 'v')
                {
                    stack.push(((x, y + 1), (x, y), len + 1));
                }
            }
            '<' => {
                if (x - 1, y) != last && matches!(map[y][x - 1], '.' | '^' | '>' | '<' | 'v') {
                    stack.push(((x - 1, y), (x, y), len + 1));
                }
            }
            '>' => {
                if (x + 1, y) != last && matches!(map[y][x + 1], '.' | '^' | '>' | '<' | 'v') {
                    stack.push(((x + 1, y), (x, y), len + 1));
                }
            }
            c => unreachable!("{c}"),
        }
    }

    Some(max_len)
}

pub fn part_two(input: &str) -> Option<u32> {
    let map: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();
    let (w, h) = (map.len(), map[0].len());

    let start = (1, 0);
    let end = (w - 2, h - 1);

    let visited = vec![false; w * h];
    let mut stack = vec![(start, visited, 0)];
    let mut max_len = 0;

    while let Some(((x, y), visited, len)) = stack.pop() {
        if (x, y) == end {
            max_len = max_len.max(len);
        }

        let mut new_visited = visited;
        new_visited[y * w + x] = true;

        match map[y][x] {
            '.' | '^' | 'v' | '<' | '>' => {
                let up = y > 0 && !new_visited[(y - 1) * w + x] && !matches!(map[y - 1][x], '#');
                let down = y < map.len() - 1
                    && !new_visited[(y + 1) * w + x]
                    && !matches!(map[y + 1][x], '#');
                let right = !new_visited[(y) * w + x + 1] && !matches!(map[y][x + 1], '#');
                let left = !new_visited[(y) * w + x - 1] && !matches!(map[y][x - 1], '#');

                let single = (up as u32 + down as u32 + right as u32 + left as u32) == 1;

                if single {
                    if up {
                        stack.push(((x, y - 1), new_visited, len + 1));
                    } else if down {
                        stack.push(((x, y + 1), new_visited, len + 1));
                    } else if left {
                        stack.push(((x - 1, y), new_visited, len + 1));
                    } else if right {
                        stack.push(((x + 1, y), new_visited, len + 1));
                    }
                } else {
                    if up {
                        stack.push(((x, y - 1), new_visited.clone(), len + 1));
                    }
                    if down {
                        stack.push(((x, y + 1), new_visited.clone(), len + 1));
                    }
                    if left {
                        stack.push(((x - 1, y), new_visited.clone(), len + 1));
                    }
                    if right {
                        stack.push(((x + 1, y), new_visited.clone(), len + 1));
                    }
                }
            }
            c => unreachable!("{c}"),
        }
    }

    Some(max_len)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(154));
    }
}
