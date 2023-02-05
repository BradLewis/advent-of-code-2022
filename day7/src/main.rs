use std::{cmp::Ordering, fs};

#[derive(Debug)]
struct FolderResult {
    size: usize,
    name: String,
}

impl FolderResult {
    fn new() -> Self {
        Self {
            size: usize::MAX,
            name: String::from(""),
        }
    }
}

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

    fn calc_sizes(&mut self) {
        let mut size = 0;
        for i in 0..self.subfolders.len() {
            self.subfolders[i].calc_sizes();
            size += self.subfolders[i].size;
        }
        let files_size: usize = self.files.iter().sum();
        self.size = size + files_size;
    }

    fn add_file(&mut self, file: usize) {
        self.files.push(file);
        self.size += file;
    }

    fn add_folder(&mut self, folder: Folder) {
        self.size += folder.size;
        self.subfolders.push(folder);
    }

    fn parse(&mut self, lines: &Vec<&str>, mut index: usize) -> usize {
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
                return index;
            }
            if line.starts_with("$ cd") {
                let temp: Vec<&str> = lines[index].split_whitespace().collect();
                let name = temp[2];
                let i = self.subfolders.iter().position(|f| f.name == name);
                if i == None {
                    let (subfolder, new_index) = Folder::from_commands(lines, index);
                    self.add_folder(subfolder);
                    index = new_index;
                    continue;
                }
                index += 1;
                index = self.subfolders[i.unwrap()].parse(lines, index);
                continue;
            }
            let x: Vec<&str> = line.split_whitespace().collect();
            let size = x[0].parse::<usize>().unwrap();
            self.add_file(size);
            index += 1;
        }
        index
    }

    fn from_commands(lines: &Vec<&str>, mut index: usize) -> (Self, usize) {
        let x: Vec<&str> = lines[index].split_whitespace().collect();
        let mut folder = Folder::new(x[2]);
        index += 1;
        index = folder.parse(lines, index);
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

    fn get_smallest_subfolder_above<'a>(
        &'a self,
        limit: usize,
        current: &'a mut FolderResult,
    ) -> &FolderResult {
        for subfolder in self.subfolders.iter() {
            println!("{}: {}", subfolder.name, subfolder.size);
            if subfolder.size < limit {
                continue;
            }
            let _ = subfolder.get_smallest_subfolder_above(limit, current);
            if (subfolder.size - limit) < (current.size - limit) {
                current.size = subfolder.size;
                current.name = String::from(&subfolder.name);
            }
        }
        current
    }
}

fn main() {
    let content = fs::read_to_string("input.txt").expect("File not found");
    let (mut folder, _) = Folder::from_commands(&content.lines().collect(), 0);
    folder.calc_sizes();

    part1(&folder);
    part2(&folder);
}

fn part1(f: &Folder) {
    let total = f.sum_size_under(100_000);

    println!("{}", total);
}

fn part2(f: &Folder) {
    let total_disk_space = 70_000_000;
    let space_required = 30_000_000;
    let space_needed = space_required - (total_disk_space - f.size);

    println!("{}: {}: {}", total_disk_space, f.size, space_needed);

    let mut result = FolderResult::new();

    let to_delete = f.get_smallest_subfolder_above(space_needed, &mut result);
    println!("{}: {}: {}", to_delete.name, to_delete.size, space_needed);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_commands() -> Result<(), String> {
        let s = fs::read_to_string("test_input.txt").expect("File not found");
        let (mut folder, _) = Folder::from_commands(&s.lines().collect(), 0);
        folder.calc_sizes();
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
        let (mut folder, _) = Folder::from_commands(&s.lines().collect(), 0);
        folder.calc_sizes();
        assert_eq!(folder.sum_size_under(100000), 95437);
        Ok(())
    }

    #[test]
    fn get_smallest_subfolder_above() -> Result<(), String> {
        let s = fs::read_to_string("test_input.txt").expect("File not found");
        let (mut folder, _) = Folder::from_commands(&s.lines().collect(), 0);
        folder.calc_sizes();
        let mut result = FolderResult::new();
        let folder_to_delete = folder.get_smallest_subfolder_above(8381165, &mut result);
        assert_eq!(folder_to_delete.name, "d");
        assert_eq!(folder_to_delete.size, 24933642);
        Ok(())
    }
}
