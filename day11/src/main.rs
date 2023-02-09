use std::fs;

struct Monkey {
    
}

fn main() {
    let s = fs::read_to_string("input.txt").expect("File not found");
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_from_commands() -> Result<(), String> {
        assert_eq!(1, 21);
        Ok(())
    }
}
