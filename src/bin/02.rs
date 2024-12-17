advent_of_code::solution!(2);

fn parse_input(input: &str) -> Vec<Vec<u64>> {
    let mut res = Vec::<Vec<u64>>::new();

    for line in input.split('\n') {
        if (line.trim().len() < 9) { print!("Skipping line {line}\n"); continue; }
        res.push(line.trim().split_whitespace().map(|s| s.parse().unwrap()).collect())
    }

    res
}

fn record_to_diff(rec: &[u64]) -> Vec<i64> {
    let mut res = Vec::<i64>::with_capacity(rec.len() - 1);
    for i in 0..(rec.len() - 2) {
        res.push((rec[i] - rec[i + 1]).try_into().unwrap());
    }
    res
}

pub fn part_one(input: &str) -> Option<u64> {
    let records = parse_input(input);

    let mut safe_count: usize = 0;

    for record in records.iter().map(|r| record_to_diff(r)) {

    }

    None
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
