struct Cpu {
    regx: i32,
}

impl Cpu {
    fn new() -> Self {
        Self { regx: 1 }
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_from_commands() -> Result<(), String> {
        Ok(())
    }
}
