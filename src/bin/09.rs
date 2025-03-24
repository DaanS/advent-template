advent_of_code::solution!(9);

fn parse_input(input: &str) -> Vec<Option<u64>> {
    let len = input.chars().filter_map(|c| c.to_digit(10)).sum::<u32>() as usize;
    let mut res = Vec::with_capacity(len);
    let mut id = 0;
    let mut free = false;

    for c in input.chars() {
        if let Some(size) = c.to_digit(10) {
            let val = if free { None } else { Some(id) };
            for  i in 0..size { res.push(val); }
            free = !free;
            if free { id += 1; }
        }
    }

    res
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut disk_map = parse_input(input);
    let mut free_idx = disk_map.iter().position(|v| v.is_none()).unwrap();
    let mut file_idx = disk_map.iter().rposition(|v| v.is_some()).unwrap();

    while free_idx < file_idx {
        disk_map.swap(free_idx, file_idx);
        while let Some(_) = disk_map[free_idx] { free_idx += 1; }
        while let None = disk_map[file_idx] { file_idx -= 1; }
    }

    Some(disk_map.iter().enumerate().filter_map(|(idx, &opt)| if let Some(val) = opt { Some(val * idx as u64) } else { None }).sum())
}

struct Segment {
    id: Option<usize>,
    start: usize,
    len: usize
}

fn parse_input_defrag(input: &str) -> Vec<Segment> {
    let mut free = false;
    let mut id = 0;
    input.chars().filter_map(|c| {
        if let Some(len) = c.to_digit(10) {
            let val = if free { None } else { Some(id) };
            free = !free;
            if free { id += 1 };
            Some(Segment{id: val, len: len as usize})
        } else { None }
    }).collect()
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
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
