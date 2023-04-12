#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
pub struct Map {
    map: Vec<Vec<u8>>,
    width: usize,
    height: usize,
}

impl Map {
    fn load(s: &str) -> Self {
        let width: usize = s.lines().map(|l| l.len()).max().unwrap();
        let map: Vec<_> = s
            .lines()
            .map(|l| {
                let mut v = vec![b' '; width];
                for (i, e) in l.chars().enumerate() {
                    v[i] = e as u8;
                }
                v
            })
            .collect();
        let height = map.len();
        Self { map, width, height }
    }

    pub fn starting_position(&self) -> Position {
        for i in 0..self.map[0].len() {
            if self.map[0][i] == b'.' {
                return Position::new(i, 0);
            }
        }
        unreachable!();
    }

    pub fn travel(&self, position: Position, direction: Direction, steps: usize) -> Position {
        Position::new(0, 0)
    }

    fn travel_right(&self, position: Position, mut steps: usize) -> Position {
        let mut x = position.x;
        while steps != 0 {
            if x + 1 > self.width {}
            steps -= 1;
        }
        Position::new(0, 0)
    }

    fn get_next_position(&self, position: Position, direction: Direction) -> Position {
        match direction {
            Direction::Up => {
                if position.y != 0 && self.map[position.y - 1][position.x] != b' ' {
                    return Position::new(position.x, position.y - 1);
                }
                for offset in 1..self.height {
                    let y = (position.y as isize - offset as isize).rem_euclid(self.height as isize)
                        as usize;
                    if self.map[y][position.x] != b' ' {
                        return Position::new(position.x, y);
                    }
                }
                unreachable!();
            }
            Direction::Down => {
                if position.y + 1 < self.height && self.map[position.y + 1][position.x] != b' ' {
                    return Position::new(position.x, position.y + 1);
                }
                for offset in 1..self.height {
                    let y = (position.y + offset).rem_euclid(self.height);
                    if self.map[y][position.x] != b' ' {
                        return Position::new(position.x, y);
                    }
                }
                unreachable!();
            }
            Direction::Left => {
                if position.x != 0 && self.map[position.y][position.x - 1] != b' ' {
                    return Position::new(position.x - 1, position.y);
                }
                for offset in 1..self.width {
                    let x = (position.x as isize - offset as isize).rem_euclid(self.width as isize)
                        as usize;
                    if self.map[position.y][x] != b' ' {
                        return Position::new(x, position.y);
                    }
                }
                unreachable!();
            }
            Direction::Right => {
                if position.x + 1 < self.width && self.map[position.y][position.x + 1] != b' ' {
                    return Position::new(position.x + 1, position.y);
                }
                for offset in 1..self.width {
                    let x = (position.x + offset).rem_euclid(self.width);
                    if self.map[position.y][x] != b' ' {
                        return Position::new(x, position.y);
                    }
                }
                unreachable!();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn get_map() -> Map {
        let s = fs::read_to_string("test_input.txt").expect("File not found");
        let (s, _) = s.split_once("\n\n").unwrap();
        Map::load(s)
    }

    #[test]
    fn test_load_map() {
        let map = get_map();
        assert_eq!(map.map[0].len(), 16);
        assert_eq!(map.map.len(), 12);
        assert_eq!(map.map[8][11], b'#');
    }

    #[test]
    fn test_starting_position() {
        let map = get_map();
        assert_eq!(map.starting_position(), Position::new(8, 0));
    }

    #[test]
    fn test_map_travel_right() {
        let map = get_map();
        let p = map.travel(Position::new(8, 0), Direction::Right, 10);
        assert_eq!(p, Position::new(10, 0));

        let p = map.travel(Position::new(10, 1), Direction::Right, 10);
        assert_eq!(p, Position::new(8, 1));
    }

    #[test]
    fn test_map_travel_left() {
        let map = get_map();
        let p = map.travel(Position::new(10, 0), Direction::Left, 10);
        assert_eq!(p, Position::new(8, 0));

        let p = map.travel(Position::new(10, 8), Direction::Left, 10);
        assert_eq!(p, Position::new(12, 8));
    }

    #[test]
    fn test_map_get_next_position() {
        let map = get_map();
        let p = map.get_next_position(Position::new(11, 1), Direction::Right);
        assert_eq!(p, Position::new(8, 1));

        let p = map.get_next_position(Position::new(11, 2), Direction::Right);
        assert_eq!(p, Position::new(8, 2));

        let p = map.get_next_position(Position::new(0, 7), Direction::Right);
        assert_eq!(p, Position::new(1, 7));

        let p = map.get_next_position(Position::new(10, 1), Direction::Left);
        assert_eq!(p, Position::new(9, 1));

        let p = map.get_next_position(Position::new(8, 2), Direction::Left);
        assert_eq!(p, Position::new(11, 2));

        let p = map.get_next_position(Position::new(0, 7), Direction::Left);
        assert_eq!(p, Position::new(11, 7));

        let p = map.get_next_position(Position::new(2, 4), Direction::Up);
        assert_eq!(p, Position::new(2, 7));

        let p = map.get_next_position(Position::new(0, 5), Direction::Up);
        assert_eq!(p, Position::new(0, 4));

        let p = map.get_next_position(Position::new(8, 0), Direction::Up);
        assert_eq!(p, Position::new(8, 11));

        let p = map.get_next_position(Position::new(2, 7), Direction::Down);
        assert_eq!(p, Position::new(2, 4));

        let p = map.get_next_position(Position::new(0, 4), Direction::Down);
        assert_eq!(p, Position::new(0, 5));

        let p = map.get_next_position(Position::new(8, 11), Direction::Down);
        assert_eq!(p, Position::new(8, 0));
    }
}
