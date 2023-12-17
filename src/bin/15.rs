advent_of_code::solution!(15);

pub fn part_one(input: &str) -> Option<u32> {
    fn hash(str: &str) -> u8 {
        let mut value: u8 = 0;

        for c in str.chars() {
            assert!(c.is_ascii());
            value = value.wrapping_add(c as u8);
            value = value.wrapping_mul(17);
        }

        value
    }

    Some(input.trim().split(',').map(|line| hash(line) as u32).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    fn hash(str: &str) -> u8 {
        let mut value: u8 = 0;

        for c in str.chars() {
            assert!(c.is_ascii());
            value = value.wrapping_add(c as u8);
            value = value.wrapping_mul(17);
        }

        value
    }

    let mut hashmap = vec![vec![]; 256];
    for op in input.trim().split(',') {
        let label = op
            .chars()
            .take_while(|c| c.is_alphabetic())
            .collect::<String>();
        let hash = hash(label.as_str()) as usize;
        match op.chars().nth(label.len()).unwrap() {
            '-' => {
                if let Some(index) = hashmap[hash].iter().position(|(l, _)| *l == label) {
                    hashmap[hash].remove(index);
                }
            }
            '=' => {
                let focal_len = op
                    .chars()
                    .nth(label.len() + 1)
                    .unwrap()
                    .to_digit(10)
                    .unwrap();
                if let Some(index) = hashmap[hash].iter().position(|(l, _)| *l == label) {
                    hashmap[hash][index] = (label, focal_len);
                } else {
                    hashmap[hash].push((label, focal_len));
                }
            }
            _ => unreachable!(),
        }
    }

    Some(
        hashmap
            .into_iter()
            .enumerate()
            .map(|(i, v)| {
                (i + 1) as u32
                    * v.into_iter()
                        .enumerate()
                        .map(|(j, (_, l))| (j as u32 + 1) * l)
                        .sum::<u32>()
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
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(145));
    }
}
