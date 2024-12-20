advent_of_code::solution!(4);

fn check_xmas(square: &Vec<Vec<char>>, x: usize, y: usize, dx: isize, dy: isize) -> bool {
    square[y.checked_add_signed(dy).unwrap()][x.checked_add_signed(dx).unwrap()] == 'M' &&
    square[y.checked_add_signed(2 * dy).unwrap()][x.checked_add_signed(2 * dx).unwrap()] == 'A' &&
    square[y.checked_add_signed(3 * dy).unwrap()][x.checked_add_signed(3 * dx).unwrap()] == 'S'
}

fn check_mas(square: &Vec<Vec<char>>, x: usize, y: usize, dx: isize, dy: isize) -> bool {
    square[y.checked_add_signed(dy).unwrap()][x.checked_add_signed(dx).unwrap()] == 'M' &&
    square[y.checked_add_signed(-dy).unwrap()][x.checked_add_signed(-dx).unwrap()] == 'S'
}

pub fn part_one(input: &str) -> Option<u64> {
    let square: Vec<Vec<char>> = input.split('\n').filter_map(|s| {
        if s.trim().len() > 0 { Some(s.chars().collect()) } else { None }
    }).collect();

    let mut xmas_count = 0;
    
    for y in 0..square.len() {
        for x in 0..square[y].len() {
            if square[y][x] == 'X' {
                if x > 2 && y > 2 && check_xmas(&square, x, y, -1, -1) { xmas_count += 1 }
                if y > 2 && check_xmas(&square, x, y, 0, -1) { xmas_count += 1 }
                if y > 2 && x < square[y].len() - 3 && check_xmas(&square, x, y, 1, -1) { xmas_count += 1 }
                if x > 2 && check_xmas(&square, x, y, -1, 0) { xmas_count += 1 }
                if x < square[y].len() - 3 && check_xmas(&square, x, y, 1, 0) { xmas_count += 1 }
                if x > 2 && y < square.len() - 3 && check_xmas(&square, x, y, -1, 1) { xmas_count += 1 }
                if y < square.len() - 3 && check_xmas(&square, x, y, 0, 1) { xmas_count += 1 }
                if y < square.len() - 3 && x < square[y].len() - 3 && check_xmas(&square, x, y, 1, 1) { xmas_count += 1 }
            }
        }
    }
    
    Some(xmas_count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let square: Vec<Vec<char>> = input.split('\n').filter_map(|s| {
        if s.trim().len() > 0 { Some(s.chars().collect()) } else { None }
    }).collect();

    let mut xmas_count = 0;
    
    for y in 1..square.len() - 1 {
        for x in 1..square[y].len() - 1 {
            if square[y][x] == 'A' {
                let mut mas_count = 0;
                for dx in [-1, 1] {
                    for dy in [-1, 1] {
                        if check_mas(&square, x, y, dx, dy) { mas_count += 1 }
                    }
                }
                assert!(mas_count <= 2);
                if mas_count == 2 { xmas_count += 1 }
            }
        }
    }
    
    Some(xmas_count)
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
