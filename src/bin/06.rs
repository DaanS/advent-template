advent_of_code::solution!(6);

#[derive(Clone, Copy)]
enum Tile {
    Blocked,
    Free(bool)
}

#[derive(Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West
}

fn parse_input(input: &str) -> ((isize, isize), Direction, Vec<Vec<Tile>>) {
    let mut map: Vec<Vec<Tile>> = Vec::new();
    let dir = Direction::North;

    let mut guard_pos = (0, 0);

    let mut y = 0;
    for line in input.split('\n') {
        if line.trim().len() == 0 { continue; }
        let mut x = 0;
        for tile in line.chars() {
            if map.len() <= x { map.push(Vec::new()); }
            map[x].push(match tile {
                '#' => Tile::Blocked,
                '.' => Tile::Free(false),
                '^' => { guard_pos = (x as isize, y as isize); Tile::Free(true) }
                _ => panic!("unexpected tile {tile}")
            });
            x += 1;
        }
        y += 1;
    }

    (guard_pos, dir, map)
}

fn get_next_pos((x, y): (isize, isize), dir: Direction) -> (isize, isize) {
    match dir {
        Direction::North => (x, y - 1),
        Direction::East => (x + 1, y),
        Direction::South => (x, y + 1),
        Direction::West => (x - 1, y)
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let ((mut guard_pos), mut dir, mut map) = parse_input(input);

    loop {
        let (new_x, new_y) = get_next_pos(guard_pos, dir);
        if (new_x < 0) || (new_y < 0) || (new_x >= map.len() as isize) || (new_y >= map[new_x as usize].len() as isize) { break; }

        match map[new_x as usize][new_y as usize] {
            Tile::Free(_) => {
                map[new_x as usize][new_y as usize] = Tile::Free(true);
                guard_pos = (new_x, new_y);
            },
            Tile::Blocked => {
                dir = match dir {
                    Direction::North => Direction::East,
                    Direction::East => Direction::South,
                    Direction::South => Direction::West,
                    Direction::West => Direction::North
                }
            }
        }
    }

    let mut count = 0;
    for line in map {
        for tile in line {
            if let Tile::Free(true) = tile { count += 1; }
        }
    }
    Some(count)
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
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
