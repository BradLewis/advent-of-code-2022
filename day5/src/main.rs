use std::fs;

#[derive(Debug, Clone)]
struct Stack {
    crate_stacks: Vec<Vec<char>>,
}

impl Stack {
    fn new() -> Self {
        Self {
            crate_stacks: Vec::new(),
        }
    }

    fn move_crates_in_block(&mut self, from: usize, to: usize, number: usize) {
        let to_remove = self.crate_stacks[from - 1].len() - number;
        let to_move: Vec<_> = self.crate_stacks[from - 1].drain(to_remove..).collect();
        self.crate_stacks[to - 1].extend_from_slice(&to_move);
    }

    fn move_crates(&mut self, from: usize, to: usize, number: usize) {
        for _ in 0..number {
            self.move_crate(from, to);
        }
    }

    fn move_crate(&mut self, from: usize, to: usize) {
        let element = self.crate_stacks[from - 1].pop().unwrap();
        self.crate_stacks[to - 1].push(element)
    }

    fn get_top_row(&self) -> String {
        let mut result = String::from("");
        for stack in self.crate_stacks.iter() {
            result.push(*stack.last().unwrap());
        }
        result
    }

    fn from_string(s: &str) -> Self {
        let mut stack = Stack::new();
        let lines: Vec<&str> = s.lines().collect();
        let last_line = lines.last().expect("Can't get last");

        for (i, item) in (1..last_line.len()).step_by(4).enumerate() {
            stack.crate_stacks.push(Vec::new());
            for line_num in (0..(lines.len() - 1)).rev() {
                if item >= lines[line_num].len() {
                    continue;
                }
                let c = lines[line_num].as_bytes()[item] as char;
                if c.is_alphabetic() {
                    stack.crate_stacks[i].push(c);
                }
            }
        }
        return stack;
    }
}

fn main() {
    let content = fs::read_to_string("input.txt").expect("File not found");
    let (first, second) = content.split_once("\n\n").expect("Failed to split");
    let stack = Stack::from_string(first);

    part1(stack.clone(), second);
    part2(stack.clone(), second);
}

fn parse_moves(line: &str) -> (usize, usize, usize) {
    let x: Vec<&str> = line.split_whitespace().collect();
    (
        x[3].parse::<usize>().unwrap(),
        x[5].parse::<usize>().unwrap(),
        x[1].parse::<usize>().unwrap(),
    )
}

fn part1(mut stack: Stack, moves: &str) {
    moves.lines().for_each(|line| {
        let (from, to, number) = parse_moves(line);
        stack.move_crates(from, to, number);
    });
    println!("{}", stack.get_top_row());
}

fn part2(mut stack: Stack, moves: &str) {
    moves.lines().for_each(|line| {
        let (from, to, number) = parse_moves(line);
        stack.move_crates_in_block(from, to, number);
    });
    println!("{}", stack.get_top_row());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_string() -> Result<(), String> {
        let content = fs::read_to_string("test_input.txt").expect("File not found");
        let (first, _) = content.split_once("\n\n").expect("Failed to split");
        let stack = Stack::from_string(first);
        assert_eq!(stack.crate_stacks.len(), 3);
        assert_eq!(stack.crate_stacks[0].len(), 2);
        assert_eq!(stack.crate_stacks[1].len(), 3);
        assert_eq!(stack.crate_stacks[2].len(), 1);
        assert_eq!(stack.crate_stacks[0][0], 'Z');
        assert_eq!(stack.crate_stacks[0][1], 'N');
        assert_eq!(stack.crate_stacks[1][0], 'M');
        assert_eq!(stack.crate_stacks[1][1], 'C');
        assert_eq!(stack.crate_stacks[1][2], 'D');
        assert_eq!(stack.crate_stacks[2][0], 'P');
        Ok(())
    }

    #[test]
    fn test_move_crate() -> Result<(), String> {
        let content = fs::read_to_string("test_input.txt").expect("File not found");
        let (first, _) = content.split_once("\n\n").expect("Failed to split");
        let mut stack = Stack::from_string(first);
        stack.move_crate(1, 2);
        assert_eq!(stack.crate_stacks[0].len(), 1);
        assert_eq!(stack.crate_stacks[1].len(), 4);
        assert_eq!(stack.crate_stacks[1][3], 'N');
        Ok(())
    }

    #[test]
    fn test_move_crates() -> Result<(), String> {
        let content = fs::read_to_string("test_input.txt").expect("File not found");
        let (first, _) = content.split_once("\n\n").expect("Failed to split");
        let mut stack = Stack::from_string(first);
        stack.move_crates(1, 2, 2);
        assert_eq!(stack.crate_stacks[0].len(), 0);
        assert_eq!(stack.crate_stacks[1].len(), 5);
        assert_eq!(stack.crate_stacks[1][4], 'Z');
        Ok(())
    }

    #[test]
    fn test_parse_moves() -> Result<(), String> {
        assert_eq!(parse_moves("move 3 from 1 to 3"), (1, 3, 3));
        assert_eq!(parse_moves("move 2 from 4 to 5"), (4, 5, 2));
        Ok(())
    }

    #[test]
    fn test_get_top_row() -> Result<(), String> {
        let content = fs::read_to_string("test_input.txt").expect("File not found");
        let (first, _) = content.split_once("\n\n").expect("Failed to split");
        let stack = Stack::from_string(first);

        assert_eq!(stack.get_top_row(), "NDP");
        Ok(())
    }

    #[test]
    fn test_move_crates_in_block() -> Result<(), String> {
        let content = fs::read_to_string("test_input.txt").expect("File not found");
        let (first, _) = content.split_once("\n\n").expect("Failed to split");
        let mut stack = Stack::from_string(first);
        stack.move_crates_in_block(1, 2, 2);

        assert_eq!(stack.crate_stacks[0].len(), 0);
        assert_eq!(stack.crate_stacks[1].len(), 5);
        assert_eq!(stack.crate_stacks[1][4], 'N');
        Ok(())
    }
}
