use std::io::stdin;

use advent_of_code::{index_offset, Grid};

advent_of_code::solution!(15);

#[derive(Default, PartialEq, Clone, Copy, Debug)]
enum Tile {
    #[default] Free,
    Wall,
    Box,
    Robot
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    North,
    East,
    South,
    West
}

fn parse_input(input: &str) -> (Grid<Tile>, Vec<Direction>) {
    let mut parts = input.split("\n\n");
    let grid_part = parts.next().unwrap();
    let grid = Grid::from_char_grid(grid_part, |c| match c {
        '.' => Tile::Free,
        '#' => Tile::Wall,
        'O' => Tile::Box,
        '@' => Tile::Robot,
        _ => panic!("unexpected tile {c}")
    });

    let dir_part = parts.next().unwrap();
    let dirs = dir_part.chars().filter_map(|c| match c {
        '^' => Some(Direction::North),
        '>' => Some(Direction::East),
        'v' => Some(Direction::South),
        '<' => Some(Direction::West),
        '\n' => None,
        _ => panic!("unexpected direction {c}")
    }).collect();

    (grid, dirs)
}

fn get_next_pos((x, y): (usize, usize), dir: Direction, (w, h): (usize, usize)) -> (usize, usize) {
    let (dx, dy) = match dir {
        Direction::North => (0, -1),
        Direction::East => (1, 0),
        Direction::South => (0, 1),
        Direction::West => (-1, 0)
    };

    index_offset((x, y), (dx, dy), (w, h)).unwrap()
}

fn try_move(grid: &mut Grid<Tile>, pos: (usize, usize), dir: Direction) -> (bool, (usize, usize)) {
    let adj_pos = get_next_pos(pos, dir, (grid.width, grid.height));
    let (can_swap, _) = match grid.at(adj_pos) {
        Some(Tile::Box) => try_move(grid, adj_pos, dir),
        Some(Tile::Wall) => (false, (0, 0)),
        Some(Tile::Free) => (true, (0, 0)),
        _ => panic!("Bad move? dir = {dir:?}, adj tile = {:?}", grid.at(adj_pos))
    };

    if can_swap {
        grid.set(adj_pos, *grid.at(pos).unwrap());
        grid.set(pos, Tile::Free);
    }

    (can_swap, if can_swap { adj_pos } else { pos })
}

fn print_grid(grid: &Grid<Tile>) {
    for (idx, cell) in grid.cells.iter().enumerate() {
        if idx % grid.width == 0 { println!(); }
        print!("{}", match cell {
            Tile::Free => '.',
            Tile::Wall => '#',
            Tile::Robot => '@',
            Tile::Box => 'O'
        });
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let (mut grid, dirs) = parse_input(input);
    let mut robo_pos = grid.position(Tile::Robot).unwrap();
    for dir in dirs {
        //print_grid(&grid);
        //println!();
        //println!("{dir:?}");
        (_, robo_pos) = try_move(&mut grid, robo_pos, dir);
    }
    
    let mut sum = 0;
    for (idx, cell) in grid.cells.iter().enumerate() {
        sum += match cell {
            Tile::Box => { 100 * (idx / grid.width) + idx % grid.width },
            _ => 0
        }
    }

    Some(sum as u64)
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
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
