use regex::Regex;

advent_of_code::solution!(14);

struct Robot {
    p: (isize, isize),
    v: (isize, isize)
}

fn parse_input(input: &str) -> Vec<Robot> {
    let robot_regex = Regex::new(r"p=([-0-9]+),([-0-9]+) v=([-0-9]+),([-0-9]+)").unwrap();
    let mut robots = Vec::new();

    for line in input.trim().split('\n') {
        let (_, [px_str, py_str, vx_str, vy_str]) = robot_regex.captures(line).unwrap().extract();
        robots.push(Robot{p: (px_str.parse::<isize>().unwrap(), py_str.parse::<isize>().unwrap()), v: (vx_str.parse::<isize>().unwrap(), vy_str.parse::<isize>().unwrap())});
    }

    robots
}

fn iterate_robots(robots: &mut Vec<Robot>, iterations: isize, w: isize, h: isize) {
    for robot in robots {
        let (px, py) = robot.p;
        let (vx, vy) = robot.v;
        robot.p = ((px + iterations * vx).rem_euclid(w), (py + iterations * vy).rem_euclid(h) );
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let (w, h) = (101, 103);
    let mut robots = parse_input(input);
    iterate_robots(&mut robots, 100, w, h);

    let mut quadrant_counts = vec![0 as u64; 4];
    for robot in robots {
        let (mid_x, mid_y) = (w / 2, h / 2);
        let (px, py) = robot.p;
        if px == mid_x || py == mid_y { continue; }

        let mut quadrant = 0;
        quadrant += if px < mid_x { 0 } else { 1 };
        quadrant += if py < mid_y { 0 } else { 2 };
        quadrant_counts[quadrant] += 1;
    }

    Some(quadrant_counts.into_iter().reduce(|a, b| a * b).unwrap() as u64)
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
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
