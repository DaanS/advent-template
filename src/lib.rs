pub mod template;

// Use this file to add helper functions and additional modules.

pub fn index_offset(x_u: usize, y_u: usize, dx: isize, dy: isize, width: usize, height: usize) -> Option<(usize, usize)> {
    let (x, y) = (x_u as isize + dx, y_u as isize + dy);
    if x >= 0 && y >= 0 && (x as usize) < width && (y as usize) < height { Some((x as usize, y as usize)) } else { None }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index_offset() {
        let w = 5;
        let h = 3;

        assert_eq!(index_offset(1, 0, -1, 0, w, h), Some((0, 0)));
        assert_eq!(index_offset(0, 0, 1, 1, w, h), Some((1, 1)));
        assert_eq!(index_offset(0, 0, -1, -1, w, h), None);
        assert_eq!(index_offset(3, 2, 2, -2, w, h), None);
        assert_eq!(index_offset(2, 3, 0, 0, w, h), None);
    }
}