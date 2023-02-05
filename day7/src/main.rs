use std::{cmp::Ordering, fs};

#[derive(Debug)]
struct Folder {
    subfolders: Vec<Folder>,
    files: Vec<usize>,
    size: usize,
    name: String,
}

impl Folder {
    fn new(name: &str) -> Self {
        Self {
            subfolders: Vec::new(),
            files: Vec::new(),
            size: 0,
            name: String::from(name),
        }
    }

    fn add_file(&mut self, file: usize) {
        self.files.push(file);
        self.size += file;
    }

    fn add_folder(&mut self, folder: Folder) {
        self.size += folder.size;
        self.subfolders.push(folder);
    }

    fn from_commands(lines: &Vec<&str>, mut index: usize) -> (Self, usize) {
        let x: Vec<&str> = lines[index].split_whitespace().collect();
        let mut folder = Folder::new(x[2]);
        index += 1;
        while index < lines.len() {
            let line = lines[index];
            if line == "$ cd /" || line == "$ ls" {
                index += 1;
                continue;
            }
            if line.starts_with("dir") {
                index += 1;
                continue;
            }
            if line == "$ cd .." {
                index += 1;
                return (folder, index);
            }
            if line.starts_with("$ cd") {
                let (subfolder, new_index) = Folder::from_commands(lines, index);
                folder.add_folder(subfolder);
                index = new_index;
                continue;
            }
            let x: Vec<&str> = line.split_whitespace().collect();
            let size = x[0].parse::<usize>().unwrap();
            folder.add_file(size);
            index += 1;
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
