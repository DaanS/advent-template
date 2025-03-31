use std::collections::VecDeque;

pub mod template;

// Use this file to add helper functions and additional modules.

pub fn index_offset((x_u, y_u): (usize, usize), (dx, dy): (isize, isize),  (width, height): (usize, usize)) -> Option<(usize, usize)> {
    let (x, y) = (x_u as isize + dx, y_u as isize + dy);
    if x >= 0 && y >= 0 && (x as usize) < width && (y as usize) < height { Some((x as usize, y as usize)) } else { None }
}
pub struct Grid<T: Default> {
    cells: Vec<T>,
    pub width: usize,
    pub height: usize
}

impl<T: Default> Grid<T> {
    pub fn new(w: usize, h: usize) -> Self { Self{ cells: Vec::with_capacity(w * h), width: w, height: h } }
    pub fn from_char_grid(input: &str, construct_fn: fn(char) -> T) -> Self {
        let mut cells = Vec::with_capacity(input.len() - input.chars().filter(|c| *c == '\n').count());
        let mut width = 0;
        let mut height = 0;
        for line in input.trim().split('\n') {
            height += 1;
            width = line.len();
            for c in line.chars() {
                cells.push(construct_fn(c));
            }
        }
        Grid{ cells, width, height }
    }

    fn index_of(&self, (x, y): (usize, usize)) -> usize { y * self.width + x }
    fn exists(&self, (x, y): (usize, usize)) -> bool { self.index_of((x, y)) < self.cells.len() }
    fn checked_index_of(&self, (x, y): (usize, usize)) -> Option<usize> { 
        if self.exists((x, y)) { Some(self.index_of((x, y))) } else { None }
    }
    pub fn at(&self, (x, y): (usize, usize)) -> Option<&T> {
        self.checked_index_of((x, y)).map(|idx| &self.cells[idx])
    }
    pub fn at_mut(&mut self, (x, y): (usize, usize)) -> Option<&mut T> {
        self.checked_index_of((x, y)).map(|idx| &mut self.cells[idx])
    }
    fn at_mut_pair(&mut self, (x1, y1): (usize, usize), (x2, y2): (usize, usize)) -> (Option<&mut T>, Option<&mut T>) {
        let idx1 = self.checked_index_of((x1, y1));
        let idx2 = self.checked_index_of((x2, y2));
        if idx1 == idx2 { (None, None) }
        else {
            unsafe {
                let vec_base = (&mut self.cells).as_mut_ptr();
                let v1 = idx1.map(|idx| vec_base.add(idx).as_mut().unwrap());
                let v2 = idx2.map(|idx| vec_base.add(idx).as_mut().unwrap());
                (v1, v2)
            }
        }
    }
    fn set(&mut self, (x, y): (usize, usize), val: T) {
        let idx = self.index_of((x, y));
        if idx >= self.cells.len() { self.cells.resize_with(idx + 1, Default::default); }
        self.cells[idx] = val;
    }

    pub fn visit_mut<F>(&mut self, start: (usize, usize), mut visit_fn: F) where F: FnMut(&T, Option<&mut T>, (usize, usize), Option<(usize, usize)>) -> bool {
        const CARDINALS: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

        if !self.exists(start) { return; }

        let mut queue = VecDeque::new();
        queue.push_back(start);

        let mut visited = Vec::new();

        while !queue.is_empty() {
            let prev_pos = queue.pop_front().unwrap();
            visited.push(prev_pos);
            for (dx, dy) in CARDINALS {
                if let Some(new_pos) = index_offset(prev_pos, (dx, dy), (self.width, self.cells.len() / self.width)) {
                    let (prev, new) = self.at_mut_pair(prev_pos, new_pos);
                    if visit_fn(prev.unwrap(), new, prev_pos, Some(new_pos)) { queue.push_back(new_pos); }
                } else {
                    visit_fn(self.at(prev_pos).unwrap(), None, prev_pos, None);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index_offset() {
        let w = 5;
        let h = 3;

        assert_eq!(index_offset((1, 0), (-1, 0), (w, h)), Some((0, 0)));
        assert_eq!(index_offset((0, 0), (1, 1), (w, h)), Some((1, 1)));
        assert_eq!(index_offset((0, 0), (-1, -1), (w, h)), None);
        assert_eq!(index_offset((3, 2), (2, -2), (w, h)), None);
        assert_eq!(index_offset((2, 3), (0, 0), (w, h)), None);
    }

    #[test]
    fn test_grid() {
        let mut g1 = Grid::new(3, 4);
        assert_eq!(g1.index_of((0, 0)), 0);
        assert_eq!(g1.index_of((2, 3)), 11);
        assert!(!g1.exists((2, 3)));
        assert_eq!(g1.checked_index_of((2, 3)), None);
        assert_eq!(g1.at((2, 3)), None);

        g1.set((2, 3), 8);
        assert!(g1.exists((2, 3)));
        assert_eq!(g1.checked_index_of((2, 3)), Some(g1.index_of((2, 3))));
        assert_eq!(g1.at((2, 3)), Some(8).as_ref());

        let mut g2 = Grid::from_char_grid("123\n456", |c| c.to_digit(10).unwrap());
        assert!(g2.exists((2, 1)));
        assert!(!g2.exists((3, 1)));
        assert!(!g2.exists((2, 2)));
        assert_eq!(g2.at((1, 1)), Some(5).as_ref());

        assert_eq!(g2.at_mut((0, 1)), Some(4).as_mut());
        *g2.at_mut((1, 0)).unwrap() = 9;
        assert_eq!(g2.at((1, 0)), Some(9).as_ref());
    }
}