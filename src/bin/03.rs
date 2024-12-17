advent_of_code::solution!(3);

use regex::Regex;

enum Op {
    Mul(u64, u64),
    Do,
    Dont
}

fn parse(input: &str) -> Vec<(usize, Op)> {
    let mul_re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let muls = mul_re.captures_iter(input).map(|captures| {
        let (_, [left, right]) = captures.extract();
        (captures.get(0).unwrap().start(), Op::Mul(left.parse::<u64>().unwrap(), right.parse::<u64>().unwrap()))
    });

    let do_re = Regex::new(r"do\(\)").unwrap();
    let dos = do_re.captures_iter(input).map(|captures| {
        (captures.get(0).unwrap().start(), Op::Do)
    });

    let dont_re = Regex::new(r"don't\(\)").unwrap();
    let donts = dont_re.captures_iter(input).map(|captures| {
        (captures.get(0).unwrap().start(), Op::Dont)
    });

    let mut res: Vec<(usize, Op)> = muls.collect();
    res.extend(dos);
    res.extend(donts);
    res.sort_by(|(left_idx, _), (right_idx, _)| left_idx.cmp(right_idx));

    res
}

pub fn part_one(input: &str) -> Option<u64> {
    let muls = parse(input);

    Some(muls.into_iter().map(|(_, op)| if let Op::Mul(left, right) = op { left * right } else { 0 } ).sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    let ops = parse(input);
    let mut on = true;

    Some(ops.into_iter().map(|(_, op)| {
        match op {
            Op::Mul(left, right) => if on { left * right } else { 0 },
            Op::Do => { on = true; 0 },
            Op::Dont => { on = false; 0 }
        }
    }).sum())
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
