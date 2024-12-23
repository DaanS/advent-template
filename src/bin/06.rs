use std::ops::{Index, IndexMut};

advent_of_code::solution!(6);

#[derive(Clone, Copy)]
struct Visited {
    north: bool,
    east: bool,
    south: bool,
    west: bool
}

impl Visited {
    fn new() -> Self {
        Visited{north: false, east: false, south: false, west: false}
    }

    fn visited_any(&self) -> bool {
        self.north || self.east || self.south || self.west
    }
}

impl Index<Direction> for Visited {
    type Output = bool;

    fn index(&self, index: Direction) -> &Self::Output {
        match index {
            Direction::North => &self.north,
            Direction::East => &self.east,
            Direction::South => &self.south,
            Direction::West => &self.west
        }
    }
}

impl IndexMut<Direction> for Visited {
    fn index_mut(&mut self, index: Direction) -> &mut Self::Output {
        match index {
            Direction::North => &mut self.north,
            Direction::East => &mut self.east,
            Direction::South => &mut self.south,
            Direction::West => &mut self.west
        }
    }
}

#[derive(Clone, Copy)]
enum Tile {
    Blocked,
    Free(Visited)
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
                '.' => Tile::Free(Visited::new()),
                '^' => { 
                    guard_pos = (x as isize, y as isize); 
                    Tile::Free(Visited{north: true, east: false, south: false, west: false})
                }
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

fn guard_path(mut guard_pos: (isize, isize), mut dir: Direction, map: &mut Vec<Vec<Tile>>) -> bool {
    loop {
        let (new_x, new_y) = get_next_pos(guard_pos, dir);
        if (new_x < 0) || (new_y < 0) || (new_x >= map.len() as isize) || (new_y >= map[new_x as usize].len() as isize) { return false; }

        match map[new_x as usize][new_y as usize] {
            Tile::Free(mut visited) => {
                if visited[dir] { return true; }
                visited[dir] = true;
                map[new_x as usize][new_y as usize] = Tile::Free(visited);
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
}

pub fn part_one(input: &str) -> Option<u64> {
    let (guard_pos, dir, mut map) = parse_input(input);

    guard_path(guard_pos, dir, &mut map);

    let mut count = 0;
    for line in map {
        for tile in line {
            if let Tile::Free(visited) = tile { 
                if visited.visited_any() { count += 1; }
            }
        }
    }
    Some(count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (guard_pos, dir, map_orig) = parse_input(input);

    let mut count = 0;

    for x in 0..map_orig.len() {
        for y in 0..map_orig[x].len() {
            let mut map = map_orig.clone();
            match map[x][y] {
                Tile::Blocked => continue,
                Tile::Free(_) => {
                    map[x][y] = Tile::Blocked;
                    if guard_path(guard_pos, dir, &mut map) { count += 1; }
                }
            }
        }
    }
    
    Some(count)
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
        assert_eq!(result, Some(6));
    }
}
