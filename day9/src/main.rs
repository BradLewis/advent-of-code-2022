use std::fs;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new() -> Self {
        Position { x: 0, y: 0 }
    }

    fn touching(&self, p: &Position) -> bool {
        if (self.x - p.x).abs() <= 1 && (self.y - p.y).abs() <= 1 {
            return true;
        }
        false
    }
}

#[derive(Debug)]
struct Rope {
    head: Position,
    tail: Position,
    tail_visited: Vec<Position>,
}

impl Rope {
    fn new() -> Self {
        Rope {
            head: Position::new(),
            tail: Position::new(),
            tail_visited: vec![Position::new()],
        }
    }

    fn move_direction_distance(&mut self, direction: &str, distance: u32) {
        for _ in 0..distance {
            self.move_direction(direction);
        }
    }

    fn move_direction(&mut self, direction: &str) {
        match direction {
            "U" => self.move_head_up(),
            "D" => self.move_head_down(),
            "L" => self.move_head_left(),
            "R" => self.move_head_right(),
            _ => (),
        }
        self.move_tail();
        self.update_tail_visited();
    }

    fn move_head_up(&mut self) {
        self.head.y -= 1;
    }

    fn move_head_down(&mut self) {
        self.head.y += 1;
    }

    fn move_head_left(&mut self) {
        self.head.x -= 1;
    }

    fn move_head_right(&mut self) {
        self.head.x += 1;
    }

    fn move_tail(&mut self) {
        if self.head.touching(&self.tail) {
            return;
        }
        if self.head.x - self.tail.x >= 1 {
            self.tail.x += 1
        } else if self.head.x - self.tail.x <= -1 {
            self.tail.x -= 1
        }
        if self.head.y - self.tail.y >= 1 {
            self.tail.y += 1
        } else if self.head.y - self.tail.y <= -1 {
            self.tail.y -= 1
        }
    }

    fn update_tail_visited(&mut self) {
        if !self.tail_visited.contains(&self.tail) {
            self.tail_visited.push(self.tail);
        }
    }
}

fn main() {
    let s = fs::read_to_string("input.txt").expect("File not found");
    let r = part1(&s);

    println!("{}", r.tail_visited.len())
}

fn part1(s: &str) -> Rope {
    let mut rope = Rope::new();
    s.lines().for_each(|l| {
        let mut parts = l.split_whitespace();
        rope.move_direction_distance(
            parts.next().unwrap(),
            parts.next().unwrap().parse::<u32>().unwrap(),
        );
    });
    rope
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_position_touching() -> Result<(), String> {
        let p = Position::new();
        assert_eq!(p.touching(&Position { x: 1, y: 1 }), true);
        assert_eq!(p.touching(&Position { x: 1, y: 10 }), false);

        let p2 = Position { x: 50, y: -100 };
        assert_eq!(p2.touching(&Position { x: 51, y: -100 }), true);
        assert_eq!(p2.touching(&Position { x: 1, y: 10 }), false);

        Ok(())
    }

    #[test]
    fn test_moving() -> Result<(), String> {
        let mut r = Rope::new();
        r.move_direction("U");
        r.move_direction("U");
        r.move_direction("L");
        r.move_direction("R");
        r.move_direction("R");
        r.move_direction("D");

        assert_eq!(r.head.x, 1);
        assert_eq!(r.head.y, -1);

        Ok(())
    }

    #[test]
    fn test_moving_tail() -> Result<(), String> {
        let mut r = Rope::new();
        r.move_direction("U");
        r.move_tail();
        assert_eq!(r.tail.x, 0);
        assert_eq!(r.tail.y, 0);

        r.move_direction("L");
        r.move_tail();
        assert_eq!(r.tail.x, 0);
        assert_eq!(r.tail.y, 0);

        r.move_direction("U");
        r.move_tail();
        assert_eq!(r.tail.x, -1);
        assert_eq!(r.tail.y, -1);

        r.move_direction("U");
        r.move_tail();
        assert_eq!(r.tail.x, -1);
        assert_eq!(r.tail.y, -2);

        Ok(())
    }

    #[test]
    fn test_tail_visited() -> Result<(), String> {
        let mut r = Rope::new();
        r.move_direction_distance("U", 2);
        assert_eq!(r.tail_visited.len(), 2);

        Ok(())
    }

    #[test]
    fn test_part1() -> Result<(), String> {
        let s = fs::read_to_string("test_input.txt").expect("File not found");
        let r = part1(&s);
        assert_eq!(r.tail_visited.len(), 13);

        Ok(())
    }
}
