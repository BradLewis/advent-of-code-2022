use std::{cmp::Ordering, fs};

#[derive(Debug, PartialEq, Eq)]
struct OutOfBoundsError;

struct Cave {
    grid: Vec<Vec<u8>>,
    max_col: usize,
    max_row: usize,
    sand_drop_x: usize,
}

impl Cave {
    fn from_string(s: &str) -> Self {
        let mut max_row: usize = 0;
        let mut min_col: usize = 500;
        let mut max_col: usize = 500;
        let paths: Vec<_> = s
            .lines()
            .map(|l| {
                let coords: Vec<_> = l
                    .split(" -> ")
                    .map(|c| {
                        let (x, y) = to_coords(&c);
                        if y > max_row {
                            max_row = y;
                        }
                        if x > max_col {
                            max_col = x;
                        } else if x < min_col {
                            min_col = x;
                        }
                        (x, y)
                    })
                    .collect();
                coords
            })
            .collect();

        let mut grid = vec![vec![b'.'; max_col - min_col + 1]; max_row + 1];
        for path in paths.iter() {
            for i in 1..path.len() {
                let (mut x, mut y) = path[i - 1];
                let (end_x, end_y) = path[i];
                let dx = get_difference(x, end_x);
                let dy = get_difference(y, end_y);
                grid[end_y][end_x - min_col] = b'#';
                while x != end_x || y != end_y {
                    grid[y][x - min_col] = b'#';
                    x = (x as isize + dx) as usize;
                    y = (y as isize + dy) as usize;
                }
            }
        }
        let sand_drop_x = 500 - min_col;
        max_col = max_col - min_col;

        Self {
            grid,
            max_col,
            max_row,
            sand_drop_x,
        }
    }

    fn print_grid(&self) {
        for y in 0..self.grid.len() {
            for x in 0..self.grid[0].len() {
                print!("{}", self.grid[y][x] as char)
            }
            print!("\n");
        }
    }

    fn drop_sand(&mut self) -> bool {
        let mut x = self.sand_drop_x;
        let mut y: usize = 0;
        let mut prev_x: usize = x;
        let mut prev_y: usize = y;

        loop {
            let next = self.next(x, y);
            match next {
                Err(OutOfBoundsError) => return false,
                Ok(_) => (x, y) = next.unwrap(),
            }
            if x == prev_x && y == prev_y {
                self.grid[y][x] = b'o';
                return true;
            }
            prev_x = x;
            prev_y = y;
        }
    }

    fn next(&self, x: usize, y: usize) -> Result<(usize, usize), OutOfBoundsError> {
        match self.check_down(x, y) {
            Ok(false) => {}
            Ok(true) => return Ok((x, y + 1)),
            _ => return Err(OutOfBoundsError),
        };
        match self.check_left(x, y) {
            Ok(false) => {}
            Ok(true) => return Ok((x - 1, y + 1)),
            _ => return Err(OutOfBoundsError),
        };
        match self.check_right(x, y) {
            Ok(false) => {}
            Ok(true) => return Ok((x + 1, y + 1)),
            _ => return Err(OutOfBoundsError),
        };
        Ok((x, y))
    }

    fn check_down(&self, x: usize, y: usize) -> Result<bool, OutOfBoundsError> {
        if self.max_row < y + 1 {
            return Err(OutOfBoundsError);
        }
        match self.grid[y + 1][x] {
            b'.' => Ok(true),
            _ => Ok(false),
        }
    }

    fn check_left(&self, x: usize, y: usize) -> Result<bool, OutOfBoundsError> {
        if x == 0 {
            return Err(OutOfBoundsError);
        }
        if self.max_row < y + 1 {
            return Err(OutOfBoundsError);
        }
        match self.grid[y + 1][x - 1] {
            b'.' => Ok(true),
            _ => Ok(false),
        }
    }

    fn check_right(&self, x: usize, y: usize) -> Result<bool, OutOfBoundsError> {
        if x == self.max_col {
            return Err(OutOfBoundsError);
        }
        if self.max_row < y + 1 {
            return Err(OutOfBoundsError);
        }
        match self.grid[y + 1][x + 1] {
            b'.' => Ok(true),
            _ => Ok(false),
        }
    }
}

fn get_difference(x1: usize, x2: usize) -> isize {
    match x1.cmp(&x2) {
        Ordering::Less => 1,
        Ordering::Equal => 0,
        Ordering::Greater => -1,
    }
}

fn to_coords(s: &str) -> (usize, usize) {
    let (x, y) = s.split_once(",").unwrap();
    (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
}

fn main() {
    let s = fs::read_to_string("input.txt").expect("File not found");
    let result = part1(&s);
    println!("{}", result);
}

fn part1(s: &str) -> u32 {
    let mut cave = Cave::from_string(&s);
    let mut total = 0;

    loop {
        let still_dropping = cave.drop_sand();
        if !still_dropping {
            return total;
        }
        total += 1;
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_load_cave() -> Result<(), String> {
        let s = fs::read_to_string("test_input.txt").expect("File not found");
        let cave = Cave::from_string(&s);

        assert_eq!(cave.max_row, 9);
        assert_eq!(cave.max_col, 9);
        assert_eq!(cave.grid[4][4], b'#');
        Ok(())
    }

    #[test]
    fn test_to_coords() -> Result<(), String> {
        let (x, y) = to_coords("498,4");

        assert_eq!(x, 498);
        assert_eq!(y, 4);
        Ok(())
    }

    #[test]
    fn test_drop_sand() -> Result<(), String> {
        let s = fs::read_to_string("test_input.txt").expect("File not found");
        let mut cave = Cave::from_string(&s);

        let result = cave.drop_sand();

        assert_eq!(result, true);
        assert_eq!(cave.grid[cave.max_row - 1][cave.sand_drop_x], b'o');
        Ok(())
    }

    #[test]
    fn test_check_down() -> Result<(), String> {
        let s = fs::read_to_string("test_input.txt").expect("File not found");
        let cave = Cave::from_string(&s);

        assert_eq!(cave.check_down(cave.sand_drop_x, 0).unwrap(), true);
        assert_eq!(cave.check_down(4, 4).unwrap(), false);
        assert_eq!(
            cave.check_down(cave.max_col, cave.max_row),
            Err(OutOfBoundsError)
        );
        Ok(())
    }

    #[test]
    fn test_check_left() -> Result<(), String> {
        let s = fs::read_to_string("test_input.txt").expect("File not found");
        let cave = Cave::from_string(&s);

        assert_eq!(cave.check_left(0, cave.max_row), Err(OutOfBoundsError));
        assert_eq!(cave.check_left(4, 4).unwrap(), true);
        assert_eq!(cave.check_left(3, 5).unwrap(), false);
        Ok(())
    }

    #[test]
    fn test_check_right() -> Result<(), String> {
        let s = fs::read_to_string("test_input.txt").expect("File not found");
        let cave = Cave::from_string(&s);

        assert_eq!(cave.check_right(cave.max_col, 4), Err(OutOfBoundsError));
        assert_eq!(cave.check_right(4, 4).unwrap(), true);
        assert_eq!(cave.check_right(3, 5).unwrap(), false);
        Ok(())
    }

    #[test]
    fn test_next() -> Result<(), String> {
        let s = fs::read_to_string("test_input.txt").expect("File not found");
        let cave = Cave::from_string(&s);

        assert_eq!(cave.next(7, 8).unwrap(), (7, 8));
        assert_eq!(
            cave.next(cave.sand_drop_x, 0).unwrap(),
            (cave.sand_drop_x, 1)
        );
        assert_eq!(cave.next(4, 4).unwrap(), (3, 5));
        Ok(())
    }

    #[test]
    fn test_part1() -> Result<(), String> {
        let s = fs::read_to_string("test_input.txt").expect("File not found");
        let result = part1(&s);

        assert_eq!(result, 24);
        Ok(())
    }
}
