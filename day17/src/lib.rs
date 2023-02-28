use std::collections::HashSet;

const WIDTH: usize = 7;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Position(isize, isize);

#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Down,
}

#[derive(Debug)]
struct Piece<'a> {
    pattern: &'a HashSet<Position>,
    position: Position,
}

#[derive(Debug)]
struct PieceFactory {
    patterns: Vec<HashSet<Position>>,
}

impl PieceFactory {
    fn from_str(s: &str) -> Self {
        Self {
            patterns: load_pieces(s),
        }
    }

    fn create(&self, index: usize) -> Piece {
        Piece {
            pattern: &self.patterns[index],
            position: Position(2, -1),
        }
    }
}

fn load_pieces(s: &str) -> Vec<HashSet<Position>> {
    let result: Vec<_> = s
        .split("\n\n")
        .map(|b| {
            HashSet::from_iter(b.lines().rev().enumerate().flat_map(|(i, l)| {
                l.chars().enumerate().filter_map(move |(j, c)| match c {
                    '#' => Some(Position(j as isize, i as isize)),
                    _ => None,
                })
            }))
        })
        .collect();
    result
}

#[derive(Debug)]
struct Chamber {
    cave: Vec<[u8; WIDTH]>,
    jet: Vec<u8>,
    height: usize,
    iteration: usize,
}

impl Chamber {
    pub fn new(s: &str) -> Self {
        Self {
            cave: Vec::new(),
            jet: s.as_bytes().to_vec(),
            height: 0,
            iteration: 0,
        }
    }

    pub fn drop_piece(&mut self, mut piece: &mut Piece) {
        for _ in 0..3 {
            self.cave.push([b'.'; 7]);
        }
        loop {
            let direction = self.jet[self.iteration % self.jet.len()];
            match direction {
                b'<' => self.move_piece(&mut piece, Direction::Right),
                _ => self.move_piece(&mut piece, Direction::Left),
            };
        }
    }

    fn move_piece(&self, piece: &mut Piece<'_>, direction: Direction) {
        match direction {
            Direction::Left => {
                for part in piece.pattern.iter() {
                    let piece_abs_y = piece.position.1 - part.1;
                    let piece_abs_x_to_check = piece.position.0 + part.0 - 1;
                    if piece_abs_x_to_check < 0 {
                        return;
                    }
                    if piece_abs_y < 0 {
                        continue;
                    }
                    let y = (self.cave.len() as isize - 1 - piece_abs_y) as usize;
                    if self.cave[y][piece_abs_x_to_check as usize] == b'#' {
                        return;
                    }
                }
                piece.position.0 -= 1;
            }
            Direction::Right => {
                for part in piece.pattern.iter() {
                    let piece_abs_y = piece.position.1 - part.1;
                    let piece_abs_x_to_check = piece.position.0 + part.0 + 1;
                    if piece_abs_x_to_check >= WIDTH as isize {
                        return;
                    }
                    if piece_abs_y < 0 {
                        continue;
                    }
                    let y = (self.cave.len() as isize - 1 - piece_abs_y) as usize;
                    if self.cave[y][piece_abs_x_to_check as usize] == b'#' {
                        return;
                    }
                }
                piece.position.0 += 1;
            }
            Direction::Down => {
                if piece.position.1 as usize == self.cave.len() - 1 {
                    return;
                }
                for part in piece.pattern.iter() {
                    let piece_abs_y_to_check =
                        ((self.cave.len() - 1) as isize - (piece.position.1 - part.1)) as usize - 1;
                    let piece_abs_x = (piece.position.0 + part.0) as usize;

                    if piece_abs_y_to_check >= self.cave.len() {
                        piece.position.1 += 1;
                        return;
                    }
                    if self.cave[piece_abs_y_to_check][piece_abs_x] == b'#' {
                        return;
                    }
                }
                piece.position.1 += 1;
            }
        };
    }
}

#[cfg(test)]
mod tests {

    use std::fs;

    use super::*;

    #[test]
    fn test_load_pieces() -> Result<(), String> {
        let s = fs::read_to_string("pieces.txt").expect("File not found");
        let pieces = load_pieces(&s);
        assert_eq!(pieces.len(), 5);
        assert_eq!(
            pieces[0],
            HashSet::from_iter(vec![
                Position(0, 0),
                Position(1, 0),
                Position(2, 0),
                Position(3, 0)
            ])
        );
        assert_eq!(
            pieces[1],
            HashSet::from_iter(vec![
                Position(1, 0),
                Position(0, 1),
                Position(1, 1),
                Position(2, 1),
                Position(1, 2),
            ])
        );
        assert_eq!(
            pieces[2],
            HashSet::from_iter(vec![
                Position(0, 0),
                Position(1, 0),
                Position(2, 0),
                Position(2, 1),
                Position(2, 2),
            ])
        );
        Ok(())
    }

    #[test]
    fn test_move_left() -> Result<(), String> {
        let s = fs::read_to_string("pieces.txt").expect("File not found");
        let piece_factory = PieceFactory::from_str(&s);
        let s = fs::read_to_string("test_input.txt").expect("File not found");
        let mut chamber = Chamber::new(&s);
        for _ in 0..3 {
            chamber.cave.push([b'.'; 7]);
        }
        chamber.cave[2][0] = b'#';
        let mut piece = piece_factory.create(0);
        chamber.move_piece(&mut piece, Direction::Left);
        assert_eq!(piece.position, Position(1, -1));
        chamber.move_piece(&mut piece, Direction::Left);
        assert_eq!(piece.position, Position(0, -1));

        piece.position = Position(1, 0);
        chamber.move_piece(&mut piece, Direction::Left);
        assert_eq!(piece.position, Position(1, 0));

        Ok(())
    }

    #[test]
    fn test_move_right() -> Result<(), String> {
        let s = fs::read_to_string("pieces.txt").expect("File not found");
        let piece_factory = PieceFactory::from_str(&s);
        let s = fs::read_to_string("test_input.txt").expect("File not found");
        let mut chamber = Chamber::new(&s);
        for _ in 0..3 {
            chamber.cave.push([b'.'; 7]);
        }
        chamber.cave[2][WIDTH - 1] = b'#';
        let mut piece = piece_factory.create(0);
        chamber.move_piece(&mut piece, Direction::Right);
        assert_eq!(piece.position, Position(3, -1));
        chamber.move_piece(&mut piece, Direction::Right);
        assert_eq!(piece.position, Position(3, -1));

        piece.position = Position(2, 0);
        chamber.move_piece(&mut piece, Direction::Right);
        assert_eq!(piece.position, Position(2, 0));

        Ok(())
    }

    #[test]
    fn test_move_down() -> Result<(), String> {
        let s = fs::read_to_string("pieces.txt").expect("File not found");
        let piece_factory = PieceFactory::from_str(&s);
        let s = fs::read_to_string("test_input.txt").expect("File not found");
        let mut chamber = Chamber::new(&s);
        for _ in 0..3 {
            chamber.cave.push([b'.'; 7]);
        }
        let mut piece = piece_factory.create(2);
        chamber.move_piece(&mut piece, Direction::Down);
        assert_eq!(piece.position, Position(2, 0));
        chamber.move_piece(&mut piece, Direction::Down);
        assert_eq!(piece.position, Position(2, 1));
        chamber.move_piece(&mut piece, Direction::Down);
        assert_eq!(piece.position, Position(2, 2));
        chamber.move_piece(&mut piece, Direction::Down);
        assert_eq!(piece.position, Position(2, 2));

        chamber.cave[1][2] = b'#';
        piece.position = Position(2, 0);
        chamber.move_piece(&mut piece, Direction::Down);
        assert_eq!(piece.position, Position(2, 0));

        Ok(())
    }
}