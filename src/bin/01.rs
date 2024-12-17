use std::iter::zip;

advent_of_code::solution!(1);

fn parse_input(input: &str, left_list: &mut Vec::<u64>, right_list: &mut Vec::<u64>) {
    for line in input.split('\n') {
        if line.trim().len() < 3 { print!("skipping line {line}"); continue; }
        let mut parts = line.trim().split_whitespace();

        left_list.push(parts.next().unwrap().parse().unwrap());
        right_list.push(parts.next().unwrap().parse().unwrap());
    }

    left_list.sort();
    right_list.sort();
}

pub fn part_one(input: &str) -> Option<u64> {
    // TODO see if counting newlines first improves parsing performance
    let mut left_list = Vec::<u64>::new();
    let mut right_list = Vec::<u64>::new();
    parse_input(input, &mut left_list, &mut right_list);

    let mut total_dist: u64 = 0;
    for (left, right) in zip(left_list, right_list) {
        total_dist += left.abs_diff(right)
    }

    Some(total_dist)
}

pub fn part_two(input: &str) -> Option<u64> {
    // TODO see if counting newlines first improves parsing performance
    let mut left_list = Vec::<u64>::new();
    let mut right_list = Vec::<u64>::new();
    parse_input(input, &mut left_list, &mut right_list);

    let mut total_sim: u64 = 0;
    for left in left_list {
        let count = right_list.iter().filter(|&&n| n == left).count();
        total_sim += left * count as u64;
    }

    Some(total_sim) 
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
