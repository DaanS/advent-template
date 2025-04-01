use std::collections::VecDeque;

use regex::Regex;

advent_of_code::solution!(13);

struct Machine {
    ax: usize,
    ay: usize,
    bx: usize,
    by: usize,
    px: usize,
    py: usize
}

fn parse_input(input: &str) -> Vec<Machine> {
    let mut res = Vec::new();

    let button_a_regex= Regex::new(r"Button A: X+(\d+), Y+(\d+)").unwrap();
    let button_b_regex = Regex::new(r"Button B: X+(\d+), Y+(\d+)").unwrap();
    let prize_regex = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();

    let mut lines: VecDeque<_> = input.split('\n').collect();
    while !lines.len() >= 3 {
        let line = lines.pop_front().unwrap();
        if line.trim().is_empty() { continue; }

        let (_, [ax_str, ay_str]) = button_a_regex.captures(line).unwrap().extract();

        let line = lines.pop_front().unwrap();
        let (_, [bx_str, by_str]) = button_b_regex.captures(line).unwrap().extract();

        let line = lines.pop_front().unwrap();
        let (_, [px_str, py_str]) = prize_regex.captures(line).unwrap().extract();

        res.push(Machine { ax: ax_str.parse().unwrap(), ay: ay_str.parse().unwrap(), 
                            bx: bx_str.parse().unwrap(), by: by_str.parse().unwrap(), 
                            px: px_str.parse().unwrap(), py: py_str.parse().unwrap() });
    }

    res
}

pub fn part_one(input: &str) -> Option<u64> {
    let machines = parse_input(input);
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
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
