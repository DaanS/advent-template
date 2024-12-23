advent_of_code::solution!(7);

fn parse_input(input: &str) -> Vec<(u64, Vec<u64>)> {
    input.split('\n').filter(|line| line.trim().len() > 0).map(|line| {
        let mut split = line.split(':');
        (split.next().unwrap().parse().unwrap(), split.next().unwrap().split(' ').filter(|val| val.trim().len() > 0).map(|val| val.trim().parse().unwrap()).collect())
    }).collect()
}

fn find_equation(res: u64, operands: &[u64]) -> bool {
    let (head, tail) = (&operands[..operands.len() - 1], operands[operands.len() - 1]);
    if head.len() > 0 {
        (res > tail && find_equation(res - tail, head)) ||
        (res % tail == 0 && find_equation(res / tail, head))
    } else {
        res == tail
    }
}

fn strip_tail(res: u64, tail: u64) -> Option<u64> {
    let res_str = res.to_string();
    let tail_str = tail.to_string();
    res_str.strip_suffix(&tail_str).and_then(|s| s.parse::<u64>().ok())
}

fn find_equation_with_concat(res: u64, operands: &[u64]) -> bool {
    let (head, tail) = (&operands[..operands.len() - 1], operands[operands.len() - 1]);
    if head.len() > 0 {
        (res > tail && find_equation_with_concat(res - tail, head)) ||
        (res % tail == 0 && find_equation_with_concat(res / tail, head)) ||
        if let Some(new_res) = strip_tail(res, tail) { find_equation_with_concat(new_res, head) } else { false }
    } else {
        res == tail
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(parse_input(input).iter().filter_map(|(result, operands)| {
        if find_equation(*result, operands) { Some(result) } else { None }
    }).sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(parse_input(input).iter().filter_map(|(result, operands)| {
        if find_equation_with_concat(*result, operands) { Some(result) } else { None }
    }).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }

    #[test]
    fn test_concat_tail() {
        assert_eq!(concat_tail(&[1, 2], 3), &[1, 23]);
    }
}
