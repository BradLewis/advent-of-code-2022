mod map;
use map::{Direction, Map, Position, Rotation};

pub struct PathFinder {
    map: Map,
}

impl PathFinder {
    pub fn new(map: Map) -> Self {
        Self { map }
    }

    pub fn parse_instructions(&self, s: &str) -> (Position, Direction) {
        let chars: Vec<_> = s.trim().chars().collect();
        let mut iteration: usize = 0;
        let mut direction = Direction::Right;
        let mut position = self.map.starting_position();
        while iteration < chars.len() {
            if chars[iteration].is_alphabetic() {
                if chars[iteration] == 'R' {
                    direction = direction.turn(Rotation::Right);
                } else {
                    direction = direction.turn(Rotation::Left);
                }
                iteration += 1;
                continue;
            }
            let mut offset = 1;
            while iteration + offset < chars.len() && chars[iteration + offset].is_numeric() {
                offset += 1;
            }
            let moves = String::from_iter(&chars[iteration..(iteration + offset)]);
            let moves = moves.parse::<usize>().unwrap();
            position = self.map.travel(position, direction, moves);
            iteration += offset;
        }
        (position, direction)
    }
}

pub fn part1(s: &str) -> usize {
    let (blueprint, instructions) = s.split_once("\n\n").unwrap();
    let map = Map::load(blueprint);
    let path_finder = PathFinder::new(map);
    let (pos, dir) = path_finder.parse_instructions(instructions);
    (pos.y + 1) * 1000 + (pos.x + 1) * 4 + dir as usize
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_parse_input() {
        let s = fs::read_to_string("test_input.txt").expect("File not found");
        let (blueprint, instructions) = s.split_once("\n\n").unwrap();
        let map = Map::load(blueprint);

        let pf = PathFinder::new(map);
        let result = pf.parse_instructions(instructions);
        assert_eq!(result, (Position::new(7, 5), Direction::Right));
    }

    #[test]
    fn test_part1() {
        let s = fs::read_to_string("test_input.txt").expect("File not found");
        let result = part1(&s);
        assert_eq!(result, 6032);
    }
}
