use std::{cmp::Ordering, collections::HashMap, hash::Hash};

advent_of_code::solution!(5);

fn parse_input(input: &str) -> (Vec<(u64, u64)>, Vec<Vec<u64>>) {
    let mut rules = Vec::new();
    let mut seqs = Vec::new();

    for line in input.split('\n') {
        if line.contains('|') {
            let mut parts = line.trim().split('|');
            rules.push((parts.next().unwrap().parse().unwrap(), parts.next().unwrap().parse().unwrap()));
        } else if line.contains(',') {
            seqs.push(line.trim().split(',').map(|s| s.parse().unwrap()).collect());
        }
    }

    (rules, seqs)
}

fn compute_befores(rules: &[(u64, u64)]) -> HashMap<u64, Vec<u64>> {
    let mut res: HashMap<u64, Vec<u64>> = HashMap::new();
    for &(before, after) in rules {
        res.entry(after).or_default().push(before);
    }
    res
}

fn comes_before(befores: &HashMap<u64, Vec<u64>>, a: &u64, b: &u64) -> bool {
    befores.get(&b).and_then(|b| Some(b.contains(&a))).unwrap_or(false)
}

fn correctify(befores: &HashMap<u64, Vec<u64>>, seq: &[u64]) -> Vec<u64> {
    let mut res = seq.to_vec();
    res.sort_by(|a, b| {
        if comes_before(befores, a, b) { Ordering::Less }
        else if comes_before(befores, b, a){ Ordering::Greater }
        else { Ordering::Equal }
    });
    res
}

pub fn part_one(input: &str) -> Option<u64> {
    let (rules, seqs) = parse_input(input);
    let befores = compute_befores(&rules);

    let mut sum = 0;

    for seq in seqs {
        let mut accept = true;
        for i in 0..seq.len() - 1 {
            if let Some(before_this) = befores.get(&seq[i]) {
                if before_this.iter().any(|b| seq[i..].contains(b)) {
                    accept = false;
                    break;
                }
            }
        }
        if accept { sum += seq[seq.len() / 2] }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (rules, seqs) = parse_input(input);
    let befores = compute_befores(&rules);

    let mut sum = 0;

    for seq in seqs {
        let mut accept = true;
        for i in 0..seq.len() - 1 {
            if let Some(before_this) = befores.get(&seq[i]) {
                if before_this.iter().any(|b| seq[i..].contains(b)) {
                    accept = false;
                    break;
                }
            }
        }
        if !accept { 
            let correct = correctify(&befores, &seq);
            sum += correct[correct.len() / 2] 
        }
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
