use std::{collections::HashMap, fs};

#[derive(Debug)]
struct Cpu {
    regx: i32,
    cycle: i32,
    commands: HashMap<i32, i32>,
    strengths: Vec<i32>,
}

impl Cpu {
    fn new() -> Self {
        Self {
            regx: 1,
            cycle: 0,
            commands: HashMap::new(),
            strengths: Vec::new(),
        }
    }

    fn run_cycle(&mut self) {
        if (self.cycle % 40 - self.regx).abs() <= 1 {
            print!("#");
        } else {
            print!(".");
        }
        self.cycle += 1;
        if self.cycle % 40 == 0 {
            print!("\n");
        }
        if (self.cycle - 20) % 40 == 0 {
            self.strengths.push(self.cycle * self.regx);
        }
        let key = self.cycle - 2;
        if !self.commands.contains_key(&key) {
            return;
        }
        let to_add = self.commands[&key];
        self.regx += to_add;
        self.commands.remove(&key);
    }

    fn process_command(&mut self, command: &str) {
        let mut s = command.split_whitespace();
        match s.next().unwrap() {
            "addx" => self.process_add(s.next().unwrap().parse::<i32>().unwrap()),
            "noop" => self.run_cycle(),
            _ => (),
        }
    }

    fn process_add(&mut self, amount: i32) {
        self.commands.insert(self.cycle, amount);
        self.run_cycle();
        self.run_cycle();
    }
}

fn main() {
    let s = fs::read_to_string("input.txt").expect("File not found");
    let result = part1(&s);

    println!("{}", result);
}

fn part1(s: &str) -> i32 {
    let mut cpu = Cpu::new();
    for line in s.lines() {
        cpu.process_command(line);
    }

    cpu.strengths.iter().sum()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_process_add() -> Result<(), String> {
        let mut cpu = Cpu::new();
        cpu.process_add(1);
        assert_eq!(cpu.regx, 2);
        Ok(())
    }

    #[test]
    fn test_part1() -> Result<(), String> {
        let s = fs::read_to_string("test_input.txt").expect("File not found");
        let result = part1(&s);
        assert_eq!(result, 13140);
        Ok(())
    }
}
