use std::fs;

fn main() {
    let content = fs::read_to_string("input.txt").expect("File not found");

    let total: u32 = content
        .split("\n")
        .map(|m| {
            let b = m.as_bytes();
            (b[0], b[2])
        })
        .map(|(opponent, you)| {
            calc_points_for_move_played(you) + calc_points_for_outcome(opponent, you)
        })
        .sum();
    println!("ans 1: {}", total);

    let total2: u32 = content
        .split("\n")
        .map(|m| {
            let b = m.as_bytes();
            (b[0], b[2])
        })
        .map(|(opponent, you)| {
            calc_points_for_move_played(get_move(opponent, you)) + calc_points_for_outcome_v2(you)
        })
        .sum();
    println!("ans 2: {}", total2);
}

fn get_move(opponent: u8, you: u8) -> u8 {
    if you == b'X' {
        let move_played = match opponent {
            b'A' => b'Z',
            b'B' => b'X',
            b'C' => b'Y',
            _ => unreachable!(),
        };
        return move_played;
    } else if you == b'Y' {
        let move_played = match opponent {
            b'A' => b'X',
            b'B' => b'Y',
            b'C' => b'Z',
            _ => unreachable!(),
        };
        return move_played;
    }
    let move_played = match opponent {
        b'A' => b'Y',
        b'B' => b'Z',
        b'C' => b'X',
        _ => unreachable!(),
    };
    return move_played;
}

fn calc_points_for_move_played(move_played: u8) -> u32 {
    match move_played {
        b'X' => 1,
        b'Y' => 2,
        b'Z' => 3,
        _ => unreachable!(),
    }
}

fn calc_points_for_outcome(opponents_move: u8, your_move: u8) -> u32 {
    if your_move == b'X' && opponents_move == b'A' {
        return 3;
    }
    if your_move == b'X' && opponents_move == b'B' {
        return 0;
    }
    if your_move == b'X' && opponents_move == b'C' {
        return 6;
    }
    if your_move == b'Y' && opponents_move == b'A' {
        return 6;
    }
    if your_move == b'Y' && opponents_move == b'B' {
        return 3;
    }
    if your_move == b'Y' && opponents_move == b'C' {
        return 0;
    }
    if your_move == b'Z' && opponents_move == b'A' {
        return 0;
    }
    if your_move == b'Z' && opponents_move == b'B' {
        return 6;
    }
    if your_move == b'Z' && opponents_move == b'C' {
        return 3;
    }
    return 0;
}

fn calc_points_for_outcome_v2(move_played: u8) -> u32 {
    match move_played {
        b'X' => 0,
        b'Y' => 3,
        b'Z' => 6,
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_winning() -> Result<(), String> {
        assert_eq!(calc_points_for_outcome(b'C', b'X'), 6);
        assert_eq!(calc_points_for_outcome(b'A', b'Y'), 6);
        assert_eq!(calc_points_for_outcome(b'B', b'Z'), 6);
        Ok(())
    }

    #[test]
    fn test_draw() -> Result<(), String> {
        assert_eq!(calc_points_for_outcome(b'C', b'Z'), 3);
        assert_eq!(calc_points_for_outcome(b'A', b'X'), 3);
        assert_eq!(calc_points_for_outcome(b'B', b'Y'), 3);
        Ok(())
    }

    #[test]
    fn test_losing() -> Result<(), String> {
        assert_eq!(calc_points_for_outcome(b'B', b'X'), 0);
        assert_eq!(calc_points_for_outcome(b'C', b'Y'), 1);
        assert_eq!(calc_points_for_outcome(b'A', b'Z'), 0);
        Ok(())
    }

    #[test]
    fn test_points() -> Result<(), String> {
        assert_eq!(get_move(b'A', b'Y'), b'X');
        assert_eq!(get_move(b'B', b'Z'), b'Z');
        assert_eq!(get_move(b'C', b'X'), b'Y');
        assert_eq!(get_move(b'C', b'Z'), b'X');
        Ok(())
    }
}
