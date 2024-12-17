use std::fs::File;
use std::io::Write;

advent_of_code::solution!(2);

fn parse_input(input: &str) -> Vec<Vec<u64>> {
    let mut res = Vec::<Vec<u64>>::new();

    for line in input.split('\n') {
        if line.trim().len() < 9 { print!("Skipping line {line}\n"); continue; }
        res.push(line.trim().split_whitespace().map(|s| s.parse().unwrap()).collect())
    }

    res
}

fn record_to_diff(rec: &[u64]) -> Vec<i64> {
    let mut res = Vec::<i64>::with_capacity(rec.len() - 1);
    for i in 0..(rec.len() - 1) {
        res.push((rec[i] as i64) - (rec[i + 1] as i64));
    }
    res
}

pub fn part_one(input: &str) -> Option<u64> {
    let records = parse_input(input);
    let mut safe_count: u64 = 0;

    for diffs in records.iter().map(|r| record_to_diff(r)) {
        if find_problem(&diffs).is_none() { safe_count += 1; }
    }

    Some(safe_count)
}

fn check_diff(diff: i64, ascending: bool) -> bool {
    return (diff > 0) == ascending &&
            diff.abs() <= 3 &&
            diff != 0
}

fn find_problem(diffs: &[i64]) -> Option<usize> {
    let ascending_count = diffs.iter().filter(|&&d| d > 0).count();
    let descending_count = diffs.iter().filter(|&&d| d < 0).count();
    let ascending = ascending_count > descending_count;

    for i in 0..diffs.len() {
        if !check_diff(diffs[i], ascending) { return Some(i) }
    }

    None
}

fn check_problem_with_dampener(diffs: &[i64], problem: usize, ascending: bool) -> Option<usize> {
    // Didn't work because you can't just remove a diff, you need to substitute the sum of 2 diffs
    find_problem(&[&diffs[..problem], &diffs[problem + 1..]].concat())
}

fn find_problem_with_dampener(diffs: &[i64]) -> Option<usize> {
    let ascending_count = diffs.iter().filter(|&&d| d > 0).count();
    let descending_count = diffs.iter().filter(|&&d| d < 0).count();
    let ascending = ascending_count > descending_count;
    if let Some(problem) = find_problem(diffs) {
        let final_problem = check_problem_with_dampener(diffs, problem, ascending);
        return final_problem;
    }
    None
}

pub fn part_two(input: &str) -> Option<u64> {
    let records = parse_input(input);
    let mut safe_count: u64 = 0;

    for rec in records {
        let diffs = record_to_diff(&rec);
        if find_problem(&diffs).is_none() { safe_count += 1 }
        else {
            for i in 0..rec.len() {
                let sub_diffs = record_to_diff(&[&rec[..i], &rec[i + 1..]].concat());
                if find_problem(&sub_diffs).is_none() { safe_count += 1; break; }
            }
        }
    }

    Some(safe_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(26));
    }
}
