use std::fs;

#[derive(Debug)]
struct Folder {
    subfolders: Vec<Folder>,
    files: Vec<usize>,
}

impl Folder {
    fn new() -> Self {
        Self {
            subfolders: Vec::new(),
            files: Vec::new(),
        }
    }

    fn total_size(&self) -> usize {
        let folders_total: usize = self.subfolders.iter().map(|f| f.total_size()).sum();
        let files_size: usize = self.files.iter().sum();
        folders_total + files_size
    }

    fn from_string(s: &str) -> Self {
        Folder::new()
    }
}

fn main() {
    let content = fs::read_to_string("input.txt").expect("File not found");

    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total_size_files() -> Result<(), String> {
        let mut folder = Folder::new();
        folder.files.append(&mut vec![10, 12, 30]);
        assert_eq!(folder.total_size(), 52);
        Ok(())
    }

    #[test]
    fn test_total_size_folders() -> Result<(), String> {
        let mut folder = Folder::new();
        folder.files.append(&mut vec![10, 12]);
        let mut subfolder = Folder::new();
        subfolder.files.append(&mut vec![1, 2]);
        folder.subfolders.push(subfolder);
        assert_eq!(folder.total_size(), 22 + 3);
        Ok(())
    }

    #[test]
    fn test_from_string() -> Result<(), String> {
        let mut folder = Folder::new();
        folder.files.append(&mut vec![10, 12, 30]);
        assert_eq!(folder.total_size(), 52);
        Ok(())
    }
}
