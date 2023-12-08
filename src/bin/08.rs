advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let instr = input
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            'L' => true,
            'R' => false,
            _ => unreachable!(),
        })
        .cycle();

    let nodes: Vec<_> = input
        .lines()
        .skip(2)
        .map(|line| {
            let node = &line[0..3];
            let left = &line[7..10];
            let right = &line[12..15];
            (node, left, right)
        })
        .collect();

    let mut node = "AAA";
    let mut steps = 0;
    for is_left in instr {
        let (_, left, right) = nodes.iter().find(|(n, _, _)| *n == node).unwrap();
        node = if is_left { left } else { right };
        steps += 1;

        if node == "ZZZ" {
            break;
        }
    }

    Some(steps)
}

pub fn part_two(input: &str) -> Option<u64> {
    let instr = input
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            'L' => true,
            'R' => false,
            _ => unreachable!(),
        })
        .cycle();

    let nodes: Vec<_> = input
        .lines()
        .skip(2)
        .map(|line| {
            let node = &line[0..3];
            let left = &line[7..10];
            let right = &line[12..15];
            (node, left, right)
        })
        .collect();

    let current_nodes: Vec<_> = nodes
        .iter()
        .filter(|(node, _, _)| node.ends_with('A'))
        .collect();

    let cycle_lengths: Vec<_> = current_nodes
        .iter()
        .map(|node| {
            let mut node = node.0;
            let mut steps = 0;
            for is_left in instr.clone() {
                let (_, left, right) = nodes.iter().find(|(n, _, _)| *n == node).unwrap();
                node = if is_left { left } else { right };
                steps += 1;

                if node.ends_with('Z') {
                    break;
                }
            }
            steps
        })
        .collect();

    fn gcd(mut n: u64, mut m: u64) -> u64 {
        while m != 0 {
            if m < n {
                let t = m;
                m = n;
                n = t;
            }
            m = m % n;
        }
        n
    }

    fn lcm(n: u64, m: u64) -> u64 {
        (n * m) / gcd(n, m)
    }

    Some(cycle_lengths.into_iter().reduce(lcm).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
