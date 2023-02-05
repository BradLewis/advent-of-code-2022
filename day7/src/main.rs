use std::{cmp::Ordering, fs};

#[derive(Debug)]
struct Folder {
    subfolders: Vec<Folder>,
    files: Vec<usize>,
    size: usize,
}

impl Folder {
    fn new() -> Self {
        Self {
            subfolders: Vec::new(),
            files: Vec::new(),
            size: 0,
        }
    }

    fn from_commands(lines: &Vec<&str>, mut index: usize) -> (Self, usize) {
        let mut folder = Folder::new();
        while index < lines.len() {
            let line = lines[index];
            index += 1;
            if line == "$ cd /" || line == "$ ls" {
                continue;
            }
            if line.starts_with("dir") {
                continue;
            }
            if line == "$ cd .." {
                return (folder, index);
            }
            if line.starts_with("$ cd") {
                let (subfolder, new_index) = Folder::from_commands(lines, index);
                folder.size += subfolder.size;
                folder.subfolders.push(subfolder);
                index = new_index;
                continue;
            }
            let x: Vec<&str> = line.split_whitespace().collect();
            let size = x[0].parse::<usize>().unwrap();
            folder.files.push(size);
            folder.size += size;
        }
        (folder, index)
    }

    fn sum_size_under(&self, limit: usize) -> usize {
        let mut total_from_subfolders = 0;
        for subfolder in self.subfolders.iter() {
            let total = subfolder.sum_size_under(limit);
            total_from_subfolders += total;
        }
        let size = self.size;
        match size.cmp(&limit) {
            Ordering::Less => size + total_from_subfolders,
            _ => total_from_subfolders,
        }
    }
}

fn main() {
    let content = fs::read_to_string("input.txt").expect("File not found");

    part1(&content);
}

fn part1(s: &str) {
    let (folder, _) = Folder::from_commands(&s.lines().collect(), 0);
    let total = folder.sum_size_under(100000);

    println!("{}", total);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_commands() -> Result<(), String> {
        let s = fs::read_to_string("test_input.txt").expect("File not found");
        let (folder, _) = Folder::from_commands(&s.lines().collect(), 0);
        assert_eq!(folder.files.len(), 2);
        assert_eq!(folder.subfolders.len(), 2);
        let sum =
            14848514 + 8504156 + 29116 + 2557 + 62596 + 584 + 4060174 + 8033020 + 5626152 + 7214296;
        assert_eq!(folder.size, sum);
        Ok(())
    }

    #[test]
    fn test_sum_size_under() -> Result<(), String> {
        let s = fs::read_to_string("test_input.txt").expect("File not found");
        let (folder, _) = Folder::from_commands(&s.lines().collect(), 0);
        assert_eq!(folder.sum_size_under(100000), 95437);
        Ok(())
    }
}
