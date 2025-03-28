advent_of_code::solution!(9);

fn parse_input(input: &str) -> Vec<Option<u64>> {
    let len = input.chars().filter_map(|c| c.to_digit(10)).sum::<u32>() as usize;
    let mut res = Vec::with_capacity(len);
    let mut id = 0;
    let mut free = false;

    for c in input.chars() {
        if let Some(size) = c.to_digit(10) {
            let val = if free { None } else { Some(id) };
            for  _ in 0..size { res.push(val); }
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
    let mut start = 0;

    input.chars().filter_map(|c| {
        if let Some(len) = c.to_digit(10) {
            let val = if free { None } else { Some(id) };
            let res = Some(Segment{id: val, start: start, len: len as usize});

            free = !free;
            if free { id += 1 };
            start += len as usize;

            res
        } else { None }
    }).collect()
}

fn print_segments(segments: &Vec<Segment>) {
    for seg in segments {
        let id = if let Some(id) = seg.id { char::from_digit(id as u32, 10).unwrap() } else { '.' };
        for _ in 0..seg.len {
            print!("{id}");
        }
    }
    println!();
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut segments = parse_input_defrag(input);
    let max_id = segments.iter().rfind(|seg| seg.id.is_some()).unwrap().id.unwrap();


    for id in (1..=max_id).rev() {
        let file_idx = segments.iter().rposition(|seg| seg.id == Some(id)).unwrap();
        if let Some(free_idx) = segments.iter().position(|seg| seg.id == None && seg.len >= segments[file_idx].len) {
            if free_idx < file_idx {
                segments[file_idx].id = None;
                segments[free_idx].id = Some(id);
                if segments[free_idx].len > segments[file_idx].len {
                    segments.insert(free_idx + 1, Segment{id: None, start: segments[free_idx].start + segments[file_idx].len, len: segments[free_idx].len - segments[file_idx].len});
                    segments[free_idx].len = segments[file_idx + 1].len;
                }
            }
        }
    }

    Some(segments.iter().filter_map(|seg| seg.id.map(|id| id * (seg.start * seg.len + (0..seg.len).sum::<usize>()))).sum::<usize>() as u64)
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
        assert_eq!(result, Some(2858));
    }
}
