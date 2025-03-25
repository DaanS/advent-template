use std::collections::HashMap;

advent_of_code::solution!(11);

fn parse_input(input: &str) -> Vec<u64> {
    input.trim().split(' ').map(|s| s.parse::<u64>().unwrap()).collect()
}

fn parse_input_map(input: &str) -> HashMap<u64, u64> {
    let mut counts = HashMap::new();
    let stones = input.trim().split(' ').map(|s| s.parse::<u64>().unwrap());
    for stone in stones {
        *counts.entry(stone).or_insert(0) += 1;
    }
    counts
}

fn process_stone(stone: u64, blinks_remaining: u8) -> u64 {
    if blinks_remaining == 0 { 1 } else {
        if stone == 0 { process_stone(1, blinks_remaining - 1) }
        else {
            let mut stone_str = stone.to_string();
            if stone_str.len() % 2 == 0 {
                let stone_str_tail = stone_str.split_off(stone_str.len() / 2);
                process_stone(stone_str.parse::<u64>().unwrap(), blinks_remaining - 1) + 
                process_stone(stone_str_tail.parse::<u64>().unwrap(), blinks_remaining - 1)
            } else {
                process_stone(stone * 2024, blinks_remaining - 1)
            }
        }
    }
}

fn process_stones_map(stones: HashMap<u64, u64>) -> HashMap<u64, u64> {
    let mut res = HashMap::new();
    for (stone, count) in stones {
        if stone == 0 { *res.entry(1).or_insert(0) += count }
        else {
            let mut stone_str = stone.to_string();
            if stone_str.len() % 2 == 0 {
                let stone_str_tail = stone_str.split_off(stone_str.len() / 2);
                *res.entry(stone_str.parse::<u64>().unwrap()).or_insert(0) += count;
                *res.entry(stone_str_tail.parse::<u64>().unwrap()).or_insert(0) += count;
            } else {
                *res.entry(stone * 2024).or_insert(0) += count;
            }
        }
    }
    res
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut stones = parse_input_map(input);
    for _ in 0..25 {
        stones = process_stones_map(stones);
    }
    Some(stones.values().sum::<u64>())
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut stones = parse_input_map(input);
    for _ in 0..75 {
        stones = process_stones_map(stones);
    }
    Some(stones.values().sum::<u64>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
