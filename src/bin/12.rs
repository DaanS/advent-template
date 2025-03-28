use advent_of_code::Grid;

advent_of_code::solution!(12);

struct Cell {
    c: char,
    visited: bool
}

impl Default for Cell {
    fn default() -> Self { Self{c: ' ', visited: false} }
}
pub fn part_one(input: &str) -> Option<u64> {
    let mut grid = Grid::from_char_grid(input, |c| Cell{c, visited: false});
    let mut sum = 0;
    for x in 0..grid.width {
        for y in 0..grid.height {
            if grid.at((x, y)).unwrap().visited { continue; }

            let mut area = 1;
            let mut perimeter = 0;
            grid.at_mut((x, y)).unwrap().visited = true;

            println!("Starting from {:?}", (x, y));
            grid.visit_mut((x, y), |old, new_opt, _, new_pos| {
                println!("Visiting {:?}", new_pos);
                // rethink how visited should work for this
                //if new_opt.is_some() && old.c == new_opt.unwrap().c { area += 1; new_opt.unwrap().visited = true; true }
                if let Some(new) = new_opt {
                    if old.c == new.c { area += 1; new.visited = true; true }
                    else { perimeter += 1; false }
                }
                else { perimeter += 1; false }
            });

            println!("Region {} with area {} and perimeter {} costs {}", grid.at((x, y)).unwrap().c, area, perimeter, area * perimeter);
            sum += area * perimeter;
        }
    }
    Some(sum)
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
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
