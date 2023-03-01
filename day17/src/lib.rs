use std::collections::{HashMap, HashSet};

const WIDTH: usize = 7;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
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
            position: Position(2, 0),
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
    height_offset: usize,
}

impl Chamber {
    pub fn new(s: &str) -> Self {
        Self {
            cave: Vec::new(),
            jet: s.as_bytes().to_vec(),
            height: 0,
            iteration: 0,
            height_offset: 0,
        }
    }

    pub fn drop_piece(&mut self, piece: &mut Piece) {
        let rows_to_add = 4 - (self.cave.len() - (self.height - self.height_offset));
        for _ in 0..rows_to_add {
            self.cave.push([b'.'; 7]);
        }
        loop {
            let direction = self.jet[self.iteration % self.jet.len()];
            match direction {
                b'>' => self.move_piece(piece, Direction::Right),
                _ => self.move_piece(piece, Direction::Left),
            };
            self.iteration += 1;
            let current_position = piece.position;
            self.move_piece(piece, Direction::Down);
            if current_position == piece.position {
                self.place_piece(piece);
                break;
            }
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

    fn place_piece(&mut self, piece: &Piece) {
        for part in piece.pattern.iter() {
            let x = (piece.position.0 + part.0) as usize;
            let y = ((self.cave.len() - 1) as isize - (piece.position.1 - part.1)) as usize;
            self.cave[y][x] = b'#';

            self.height = self.height.max(y + 1 + self.height_offset);
        }
    }

    fn print(&self) {
        for y in (0..self.cave.len()).rev() {
            print!("|");
            for x in 0..WIDTH {
                print!("{}", self.cave[y][x] as char);
            }
            println!("|");
        }
        println!("_________");
    }

    fn get_cave_mask(&self, num_rows: usize) -> u128 {
        let mut mask = 0;
        if self.height == 0 {
            return u128::MAX;
        }
        for i in 0..num_rows {
            let row_number = (self.height - 1) as isize - i as isize;
            if ((self.height - 1) as isize - i as isize) < 0 {
                mask |= 0b1111111 << (i * WIDTH);
                continue;
            }
            for j in 0..WIDTH {
                if self.cave[row_number as usize][j] == b'#' {
                    mask |= 0b1 << (i * WIDTH + j)
                }
            }
        }
        mask
    }
}

pub fn check_height_after(
    chamber_str: &str,
    pieces_str: &str,
    drop_count: usize,
    rows_to_check: usize,
) -> usize {
    let piece_factory = PieceFactory::from_str(pieces_str);
    let mut chamber = Chamber::new(chamber_str);
    let mut cache: HashMap<(usize, usize, u128), (usize, usize)> = HashMap::new();

    let mut drop_number = 0;
    let mut found_cycle = false;
    while drop_number < drop_count {
        if !found_cycle {
            let bitmask = chamber.get_cave_mask(rows_to_check);
            if cache.contains_key(&(
                drop_number % 5,
                chamber.iteration % chamber.jet.len(),
                bitmask,
            )) {
                found_cycle = true;
                println!(
                    "Found combination in cache: {} - {}:{}",
                    chamber.iteration,
                    drop_number % 5,
                    chamber.iteration % chamber.jet.len()
                );
                let (starting_height, starting_drop) = cache
                    .get(&(
                        drop_number % 5,
                        chamber.iteration % chamber.jet.len(),
                        bitmask,
                    ))
                    .unwrap();
                let ending_height = chamber.height;
                let ending_drop = drop_number + 1;
                let delta_height = ending_height - starting_height;
                let delta_drop = ending_drop - starting_drop;
                let remaining_drops = drop_count - drop_number;
                let loops = remaining_drops / delta_drop;
                let offset = remaining_drops % delta_drop;
                chamber.height = ending_height + delta_height * loops;
                drop_number = drop_count - offset;
                chamber.height_offset = delta_height * loops;
            }
            cache.insert(
                (
                    drop_number % 5,
                    chamber.iteration % chamber.jet.len(),
                    bitmask,
                ),
                (chamber.height, drop_number + 1),
            );
        }
        let mut piece = piece_factory.create(drop_number % 5);
        chamber.drop_piece(&mut piece);
        drop_number += 1;
    }
    chamber.height
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
        chamber.cave[1][0] = b'#';
        let mut piece = piece_factory.create(0);
        chamber.move_piece(&mut piece, Direction::Left);
        assert_eq!(piece.position, Position(1, 0));
        chamber.move_piece(&mut piece, Direction::Left);
        assert_eq!(piece.position, Position(0, 0));

        piece.position = Position(1, 1);
        chamber.move_piece(&mut piece, Direction::Left);
        assert_eq!(piece.position, Position(1, 1));

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
        chamber.cave[1][WIDTH - 1] = b'#';
        let mut piece = piece_factory.create(0);
        chamber.move_piece(&mut piece, Direction::Right);
        assert_eq!(piece.position, Position(3, 0));
        chamber.move_piece(&mut piece, Direction::Right);
        assert_eq!(piece.position, Position(3, 0));

        piece.position = Position(2, 1);
        chamber.move_piece(&mut piece, Direction::Right);
        assert_eq!(piece.position, Position(2, 1));

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
        let mut piece = piece_factory.create(0);
        chamber.move_piece(&mut piece, Direction::Down);
        assert_eq!(piece.position, Position(2, 1));
        chamber.move_piece(&mut piece, Direction::Down);
        assert_eq!(piece.position, Position(2, 2));
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

    #[test]
    fn test_drop_piece() -> Result<(), String> {
        let s = fs::read_to_string("pieces.txt").expect("File not found");
        let piece_factory = PieceFactory::from_str(&s);
        let s = fs::read_to_string("test_input.txt").expect("File not found");
        let mut chamber = Chamber::new(&s);

        let mut piece = piece_factory.create(0);
        chamber.drop_piece(&mut piece);
        chamber.print();
        assert_eq!(chamber.height, 1);
        assert_eq!(chamber.cave.len(), 4);

        let mut piece = piece_factory.create(1);
        chamber.drop_piece(&mut piece);
        chamber.print();
        assert_eq!(chamber.height, 4);
        assert_eq!(chamber.cave.len(), 5);

        let mut piece = piece_factory.create(2);
        chamber.drop_piece(&mut piece);
        chamber.print();
        assert_eq!(chamber.height, 6);
        assert_eq!(chamber.cave.len(), 8);
        Ok(())
    }

    #[test]
    fn test_part1() -> Result<(), String> {
        let pieces_str = fs::read_to_string("pieces.txt").expect("File not found");
        let chamber_str = fs::read_to_string("test_input.txt").expect("File not found");

        let result = check_height_after(&chamber_str, &pieces_str, 2022, 18);
        assert_eq!(result, 3068);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<(), String> {
        let pieces_str = fs::read_to_string("pieces.txt").expect("File not found");
        let chamber_str = fs::read_to_string("test_input.txt").expect("File not found");

        let result = check_height_after(&chamber_str, &pieces_str, 1_000_000_000_000, 18);
        assert_eq!(result, 1514285714288);
        Ok(())
    }

    #[test]
    fn test_get_bit_mask() -> Result<(), String> {
        let pieces_str = fs::read_to_string("pieces.txt").expect("File not found");
        let chamber_str = fs::read_to_string("test_input.txt").expect("File not found");

        let piece_factory = PieceFactory::from_str(&pieces_str);
        let mut chamber = Chamber::new(&chamber_str);

        for i in 0..3 {
            let mut piece = piece_factory.create(i);
            chamber.drop_piece(&mut piece);
        }
        let bitmask = chamber.get_cave_mask(2);

        assert_eq!(bitmask, 0b0000100_0000100);
        Ok(())
    }
}
