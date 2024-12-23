use std::collections::HashMap;

use advent_of_code::{index_offset, template};

advent_of_code::solution!(8);

fn parse_input(input: &str) -> (usize, usize, HashMap<char, Vec<(usize, usize)>>) {
    let mut height = 0;
    let mut width = 0;
    let mut map = HashMap::new();
    for line in input.split('\n').filter(|line| line.trim().len() > 0) {
        width = line.len(); // XXX assume ascii, so that length in bytes equals length in chars
        let mut x = 0;
        for c in line.chars() {
            if c != '.' { 
                map.entry(c).or_insert(Vec::new()).push((height, x));
            }
            x += 1;
        }
        height += 1;
    }
    (width, height, map)
}

fn mark_antinodes((ax_u, ay_u): (usize, usize), (bx_u, by_u): (usize, usize), map: &mut Vec<Vec<bool>>) {
    let (ax, ay, bx, by) = (ax_u as isize, ay_u as isize, bx_u as isize, by_u as isize);
    let (dx, dy) = (ax - bx, ay - by); // from b to a
    let (w, h) = (map.len(), map[0].len());
    if let Some((c1x, c1y)) = index_offset(ax_u, ay_u, dx, dy, w, h) {
        map[c1x][c1y] = true;
    }
    if let Some((c2x, c2y)) = index_offset(ax_u, ay_u, -2 * dx, -2 * dy, w, h) {
        map[c2x][c2y] = true;
    }
}

fn mark_antinodes_harmonic((ax_u, ay_u): (usize, usize), (bx_u, by_u): (usize, usize), map: &mut Vec<Vec<bool>>) {
    let (ax, ay, bx, by) = (ax_u as isize, ay_u as isize, bx_u as isize, by_u as isize);
    let (dx, dy) = (ax - bx, ay - by); // from b to a
    let (w, h) = (map.len(), map[0].len());
    let mut n = 0;
    while let Some((cx, cy)) = index_offset(ax_u, ay_u, n * dx, n * dy, w, h) {
        map[cx][cy] = true;
        n += 1;
    }
    n = 1;
    while let Some((cx, cy)) = index_offset(ax_u, ay_u, -n * dx, -n * dy, w, h) {
        map[cx][cy] = true;
        n += 1;
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let (width, height, antennas) = parse_input(input);
    let mut map: Vec<Vec<bool>> = vec![vec![false; height]; width];

    for positions in antennas.values() {
        for i in 0..positions.len() {
            for j in i + 1..positions.len() {
                mark_antinodes(positions[i], positions[j], &mut map)
            }
        }
    }

    Some(map.iter().flatten().filter(|&&v| v).count() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (width, height, antennas) = parse_input(input);
    let mut map: Vec<Vec<bool>> = vec![vec![false; height]; width];

    for positions in antennas.values() {
        for i in 0..positions.len() {
            for j in i + 1..positions.len() {
                mark_antinodes_harmonic(positions[i], positions[j], &mut map)
            }
        }
    }

    Some(map.iter().flatten().filter(|&&v| v).count() as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }

    #[test]
    fn test_mark_antinodes() {
        let width = 6;
        let height = 3;
        let mut map = vec![vec![false; height]; width];
        mark_antinodes((1, 1), (3, 1), &mut map);
        let count = map.iter().flatten().filter(|&&v| v).count();
        assert_eq!(count, 1);
        mark_antinodes((1, 0), (2, 0), &mut map);
        let count = map.iter().flatten().filter(|&&v| v).count();
        assert_eq!(count, 3);
    }
}
