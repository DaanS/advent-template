use advent_of_code::{index_offset, Grid};

advent_of_code::solution!(15);

#[derive(Default, PartialEq)]
enum Tile {
    #[default] Free,
    Wall,
    Box,
    Robot
}

enum Direction {
    North,
    East,
    South,
    West
}

fn parse_input(input: &str) -> (Grid<Tile>, Vec<Direction>) {
    let mut parts = input.split("\n\n");
    let grid_part = parts.next().unwrap();
    let grid = Grid::from_char_grid(input, |c| match c {
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
        '_' => panic!("unexpected direction {c}")
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

fn try_move(grid: &mut Grid<Tile>, (rx, ry): (usize, usize), dir: Direction) -> Tile {
    let next_pos = get_next_pos(_, dir, (grid.width, grid.height));
    let next_tile = match grid.at(next_pos) {
        Some(Tile::Box) => try_move(grid, next_pos, dir),
        Some(Tile::Wall) => Tile::Wall,
        Some(Tile::Free) => Tile::Free,
        _ => panic!("Bad move?")
    };

    if next_tile == Tile::Free { 
        grid.set(next_pos, *grid.at((rx, ry)).unwrap());
        grid.set((rx, ry), Tile::Free);
    }

    next_tile
}

pub fn part_one(input: &str) -> Option<u64> {
    let (mut grid, dirs) = parse_input(input);
    let (mut rx, mut ry) = grid.position(Tile::Robot).unwrap();
    for dir in dirs {
        //try_move((rx, ry), dir)
    }
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
