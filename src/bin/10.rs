use std::collections::{HashSet, VecDeque};

use advent_of_code::index_offset;

advent_of_code::solution!(10);

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    let mut map = Vec::new();

    for line in input.split('\n') {
        for (x, tile) in line.chars().enumerate() {
            if map.len() <= x { map.push(Vec::new()); }
            map[x].push(tile.to_digit(10).unwrap() as u8);
        }
    }

    map
}

fn topo_iterate(map: &Vec<Vec<u8>>, start: (usize, usize), continue_fn: fn(u8, u8) -> bool, goal_fn: fn(u8) -> bool) -> Vec<(usize, usize)> {
    let mut queue = VecDeque::new();
    queue.push_back(start);

    let mut res = Vec::new();

    let (w, h) = (map.len(), map[0].len());
    let cardinals = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];

    while !queue.is_empty() {
        let (prev_x, prev_y) = queue.pop_front().unwrap();
        let prev_val = map[prev_x][prev_y];

        for (d_x, d_y) in cardinals.iter().cloned() {
            if let Some((new_x, new_y)) = index_offset(prev_x, prev_y, d_x, d_y, w, h) {
                let new_val = map[new_x][new_y];
                if continue_fn(prev_val, new_val) { 
                    if goal_fn(new_val) { res.push((new_x, new_y)); }
                    else { queue.push_back((new_x, new_y)); }
                };
            }
        }
    }

    res
}

pub fn part_one(input: &str) -> Option<u64> {
    let map = parse_input(input);

    let mut sum = 0;

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[x][y] == 0 {
                let mut peaks = topo_iterate(&map, (x, y), |prev_val, new_val| new_val == prev_val + 1, |new_val| new_val == 9);
                peaks.sort();
                peaks.dedup();
                sum += peaks.len();
            }
        }
    }

    Some(sum as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let map = parse_input(input);

    let mut sum = 0;

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[x][y] == 0 {
                sum += topo_iterate(&map, (x, y), |prev_val, new_val| new_val == prev_val + 1, |new_val| new_val == 9).len();
            }
        }
    }

    Some(sum as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
