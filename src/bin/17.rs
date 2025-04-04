use regex::Regex;

advent_of_code::solution!(17);

#[repr(u8)]
enum Op {
    adv(u8) = 0,
    bxl(u8) = 1,
    bst(u8) = 2,
    jnz(u8) = 3,
    bxv(u8) = 4,
    out(u8) = 5,
    bdv(u8) = 6,
    cdv(u8) = 7
}

struct Machine {
    a: isize,
    b: isize,
    c: isize,
    program: Vec<Op>
}

fn parse_input(input: &str) -> Machine {
    let register_regex = Regex::new(r"Register .: (\d+)").unwrap();
    let program_regex = Regex::new(r"Program: (.+)").unwrap();

    let mut lines = input.split('\n');

    let mut line = lines.next().unwrap();
    let (_, [a_str]) = register_regex.captures(line).unwrap().extract();

    let line = lines.next().unwrap();
    let (_, [b_str]) = register_regex.captures(line).unwrap().extract();

    let line = lines.next().unwrap();
    let (_, [c_str]) = register_regex.captures(line).unwrap().extract();

    lines.next();
    let line = lines.next().unwrap();
    let (_, [prog_str]) = program_regex.captures(line).unwrap().extract();

    let prog_bin: Vec<u8> = prog_str.split(',').map(|c| c.parse::<u8>().unwrap()).collect();

    Machine {
        a: a_str.parse().unwrap(),
        b: b_str.parse().unwrap(),
        c: c_str.parse().unwrap(),
        program: prog_bin.chunks(2).map(|chunk| if let &[code, operand] = chunk {
            (code as Op)(operand)
        })
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    None
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
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
