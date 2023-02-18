use std::cmp::Ordering;

#[derive(Debug, PartialEq)]
enum OutOfBoundsError {
    Left,
    Right,
    Bottom,
}

struct Cave {
    grid: Vec<Vec<u8>>,
    max_col: usize,
    max_row: usize,
    sand_drop_x: usize,
    infinite_width: bool,
}

#[derive(Debug)]
struct Sand {
    dropped: bool,
    x: usize,
    y: usize,
}

impl Sand {
    fn new(dropped: bool, x: usize, y: usize) -> Self {
        Self { dropped, x, y }
    }
}

impl Cave {
    fn from_string(s: &str, infinite_width: bool) -> Self {
        let mut max_row: usize = 0;
        let mut min_col: usize = 500;
        let mut max_col: usize = 500;
        let paths: Vec<_> = s
            .lines()
            .map(|l| {
                let coords: Vec<_> = l
                    .split(" -> ")
                    .map(|c| {
                        let (x, y) = to_coords(c);
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

        max_row += match infinite_width {
            true => 2,
            false => 0,
        };

        let mut grid = vec![vec![b'.'; max_row + 1]; max_col - min_col + 1];
        for path in paths.iter() {
            for i in 1..path.len() {
                let (mut x, mut y) = path[i - 1];
                let (end_x, end_y) = path[i];
                let dx = get_difference(x, end_x);
                let dy = get_difference(y, end_y);
                grid[end_x - min_col][end_y] = b'#';
                while x != end_x || y != end_y {
                    grid[x - min_col][y] = b'#';
                    x = (x as isize + dx) as usize;
                    y = (y as isize + dy) as usize;
                }
            }
        }

        if infinite_width {
            (0..grid.len()).for_each(|x| {
                let len = grid[x].len();
                grid[x][len - 1] = b'#';
            });
        }
        let sand_drop_x = 500 - min_col;
        max_col -= min_col;

        Self {
            grid,
            max_col,
            max_row,
            sand_drop_x,
            infinite_width,
        }
    }

    fn print_grid(&self) {
        for y in 0..self.grid[0].len() {
            for x in 0..self.grid.len() {
                print!("{}", self.get_grid_value(x, y) as char)
            }
            println!();
        }
    }

    fn get_grid_value(&self, x: usize, y: usize) -> u8 {
        self.grid[x][y]
    }
    fn set_grid_value(&mut self, x: usize, y: usize, value: u8) {
        self.grid[x][y] = value;
    }

    fn drop_sand(&mut self) -> Sand {
        let mut x = self.sand_drop_x;
        let mut y: usize = 0;
        let mut prev_x: usize = x;
        let mut prev_y: usize = y;

        loop {
            let next = self.next(x, y);
            match next {
                Err(OutOfBoundsError::Bottom) => return Sand::new(false, 0, 0),
                Err(OutOfBoundsError::Left) => match self.infinite_width {
                    false => return Sand::new(false, 0, 0),
                    true => return self.increase_width(true),
                },
                Err(OutOfBoundsError::Right) => match self.infinite_width {
                    false => return Sand::new(false, 0, 0),
                    true => return self.increase_width(false),
                },
                Ok(_) => (x, y) = next.unwrap(),
            };
            if x == prev_x && y == prev_y {
                self.set_grid_value(x, y, b'o');
                return Sand::new(true, x, y);
            }
            prev_x = x;
            prev_y = y;
        }
    }

    fn next(&self, x: usize, y: usize) -> Result<(usize, usize), OutOfBoundsError> {
        match self.check_down(x, y) {
            Ok(false) => {}
            Ok(true) => return Ok((x, y + 1)),
            Err(e) => return Err(e),
        };
        match self.check_left(x, y) {
            Ok(false) => {}
            Ok(true) => return Ok((x - 1, y + 1)),
            Err(e) => return Err(e),
        };
        match self.check_right(x, y) {
            Ok(false) => {}
            Ok(true) => return Ok((x + 1, y + 1)),
            Err(e) => return Err(e),
        };
        Ok((x, y))
    }

    fn check_down(&self, x: usize, y: usize) -> Result<bool, OutOfBoundsError> {
        if self.max_row < y + 1 {
            return Err(OutOfBoundsError::Bottom);
        }
        Ok(self.is_empty(x, y + 1))
    }

    fn check_left(&self, x: usize, y: usize) -> Result<bool, OutOfBoundsError> {
        if x == 0 {
            return Err(OutOfBoundsError::Left);
        }
        if self.max_row < y + 1 {
            return Err(OutOfBoundsError::Bottom);
        }
        Ok(self.is_empty(x - 1, y + 1))
    }

    fn check_right(&self, x: usize, y: usize) -> Result<bool, OutOfBoundsError> {
        if x == self.max_col {
            return Err(OutOfBoundsError::Right);
        }
        if self.max_row < y + 1 {
            return Err(OutOfBoundsError::Bottom);
        }
        Ok(self.is_empty(x + 1, y + 1))
    }

    fn is_empty(&self, x: usize, y: usize) -> bool {
        matches!(self.get_grid_value(x, y), b'.')
    }

    fn increase_width(&mut self, left: bool) -> Sand {
        let mut col = vec![b'.'; self.max_row + 1];
        self.max_col += 1;
        col[self.max_row - 1] = b'o';
        col[self.max_row] = b'#';
        match left {
            true => {
                self.grid.insert(0, col);
                self.sand_drop_x += 1;
                Sand::new(true, 0, self.max_row - 1)
            }
            false => {
                self.grid.push(col);
                Sand::new(true, self.max_col, self.max_row - 1)
            }
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
    let (x, y) = s.split_once(',').unwrap();
    (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
}

pub fn part1(s: &str) -> u32 {
    let mut cave = Cave::from_string(s, false);
    let mut total = 0;

    loop {
        let sand = cave.drop_sand();
        if !sand.dropped {
            return total;
        }
        total += 1;
    }
}

pub fn part2(s: &str) -> u32 {
    let mut cave = Cave::from_string(s, true);
    let mut total = 0;

    loop {
        let sand = cave.drop_sand();
        total += 1;

        if sand.x == cave.sand_drop_x && sand.y == 0 {
            return total;
        }
    }
}

#[cfg(test)]
mod tests {

    use std::fs;

    use super::*;

    #[test]
    fn test_load_cave() -> Result<(), String> {
        let s = fs::read_to_string("test_input.txt").expect("File not found");
        let cave = Cave::from_string(&s, false);

        assert_eq!(cave.max_row, 9);
        assert_eq!(cave.max_col, 9);
        assert_eq!(cave.get_grid_value(4, 4), b'#');
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
        let mut cave = Cave::from_string(&s, false);

        let result = cave.drop_sand();

        assert!(result.dropped);
        assert_eq!(
            cave.get_grid_value(cave.sand_drop_x, cave.max_row - 1),
            b'o'
        );
        Ok(())
    }

    #[test]
    fn test_check_down() -> Result<(), String> {
        let s = fs::read_to_string("test_input.txt").expect("File not found");
        let cave = Cave::from_string(&s, false);

        assert!(cave.check_down(cave.sand_drop_x, 0).unwrap());
        assert!(!cave.check_down(4, 4).unwrap());
        assert!(!cave.check_down(7, 8).unwrap());
        assert_eq!(
            cave.check_down(cave.max_col, cave.max_row),
            Err(OutOfBoundsError::Bottom)
        );
        Ok(())
    }

    #[test]
    fn test_check_left() -> Result<(), String> {
        let s = fs::read_to_string("test_input.txt").expect("File not found");
        let cave = Cave::from_string(&s, false);

        assert_eq!(
            cave.check_left(0, cave.max_row),
            Err(OutOfBoundsError::Left)
        );
        assert!(cave.check_left(4, 4).unwrap());
        assert!(!cave.check_left(3, 5).unwrap());
        Ok(())
    }

    #[test]
    fn test_check_right() -> Result<(), String> {
        let s = fs::read_to_string("test_input.txt").expect("File not found");
        let cave = Cave::from_string(&s, false);

        assert_eq!(
            cave.check_right(cave.max_col, 4),
            Err(OutOfBoundsError::Right)
        );
        assert!(cave.check_right(4, 4).unwrap());
        assert!(!cave.check_right(3, 5).unwrap());
        Ok(())
    }

    #[test]
    fn test_next() -> Result<(), String> {
        let s = fs::read_to_string("test_input.txt").expect("File not found");
        let cave = Cave::from_string(&s, false);
        cave.print_grid();

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

    #[test]
    fn test_increase_width() -> Result<(), String> {
        let s = fs::read_to_string("test_input.txt").expect("File not found");
        let mut cave = Cave::from_string(&s, true);

        let sand_drop_x = cave.sand_drop_x;
        let max_col = cave.max_col;

        cave.increase_width(true);

        let mut result = vec![b'.'; cave.max_row - 1];
        result.push(b'o');
        result.push(b'#');

        assert_eq!(cave.grid[0], result);
        assert_eq!(cave.sand_drop_x, sand_drop_x + 1);
        assert_eq!(cave.max_col, max_col + 1);

        let sand_drop_x = cave.sand_drop_x;
        let max_col = cave.max_col;
        cave.increase_width(false);

        let mut result = vec![b'.'; cave.max_row - 1];
        result.push(b'o');
        result.push(b'#');

        let len = cave.grid.len();
        assert_eq!(cave.grid[len - 1], result);
        assert_eq!(cave.sand_drop_x, sand_drop_x);
        assert_eq!(cave.max_col, max_col + 1);

        Ok(())
    }

    #[test]
    fn test_part2() -> Result<(), String> {
        let s = fs::read_to_string("test_input.txt").expect("File not found");
        let result = part2(&s);

        assert_eq!(result, 93);
        Ok(())
    }
}
