use std::collections::VecDeque;

use regex::Regex;

advent_of_code::solution!(13);

struct Machine {
    ax: isize,
    ay: isize,
    bx: isize,
    by: isize,
    px: isize,
    py: isize
}

fn parse_input(input: &str) -> Vec<Machine> {
    let mut res = Vec::new();

    let button_a_regex= Regex::new(r"Button A: X\+(\d+), Y\+(\d+)").unwrap();
    let button_b_regex = Regex::new(r"Button B: X\+(\d+), Y\+(\d+)").unwrap();
    let prize_regex = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();

    let mut lines: VecDeque<_> = input.split('\n').collect();
    while lines.len() >= 3 {
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

///
/// a*ax + b*bx = px
/// a*ay + b*by = py
/// ==
/// (ax, bx)(a)   (px)
/// (ay, by)(b) = (py)

///
/// a*ax + b*bx = px
/// a*ax = px - b*bx
/// a = (px - b*bx) / ax
/// 
/// ((px - b*bx) / ax * ay) + b*by = py
/// (px*ay - b*bx*ay + b*ax*by) / ax = py
/// px*ay / ax - b * (bx*ay + ax*by) / ax = py
/// -b * (bx*ay + ax*by) / ax = py - px*ay / ax
/// -b * (bx*ay + ax*by) = ax*py - px*ay
/// -b = (ax*py - px*ay) / (bx*ay + ax*by)
/// 
fn solve_machine(machine: &Machine, max: Option<usize>) -> usize {
    //let a_max = (machine.px / machine.ax).min(machine.py / machine.ay).min(max.unwrap_or(usize::MAX));
    //let mut min_cost = usize::MAX;

    //for a in 0..=a_max {
    //    let (rx, ry) = (machine.px - a * machine.ax, machine.py - a * machine.ay);
    //    let b = rx / machine.bx;
    //    if (rx == b * machine.bx) && (ry == b * machine.by) { 
    //        let cost = 3 * a + b;
    //        if cost < min_cost { min_cost = cost; }
    //    }
    //}

    //if min_cost == usize::MAX { 0 } else { min_cost }
    if machine.bx + machine.ax * machine.by == 0 { return 0; }
    let b = -(machine.ax * machine.py - machine.px * machine.ay) / (machine.bx * machine.ay + machine.ax * machine.by);
    let a = (machine.px - b * machine.bx) / machine.ax;
}

pub fn part_one(input: &str) -> Option<u64> {
    let machines = parse_input(input);
    Some(machines.iter().map(|machine| solve_machine(machine, Some(100))).sum::<usize>() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut machines = parse_input(input);
    for machine in machines.iter_mut() { machine.px += 10000000000000; machine.py += 10000000000000; }
    Some(machines.iter().map(|machine| solve_machine(machine, None)).sum::<usize>() as u64)
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
