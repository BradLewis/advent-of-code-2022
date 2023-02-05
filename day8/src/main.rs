use std::{cmp, fs};

struct TreePatch {
    trees: Vec<Vec<u32>>,
}

impl TreePatch {
    fn load(s: &str) -> Self {
        let patch: Vec<Vec<_>> = s
            .lines()
            .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
            .collect();

        TreePatch { trees: patch }
    }

    fn get_visible_count(&self) -> usize {
        let height = self.trees.len();
        let width = self.trees[0].len();
        let mut visible_count = 0;
        for i in 0..height {
            for j in 0..width {
                if i == 0 || i == height - 1 {
                    visible_count += 1;
                    continue;
                } else if j == 0 || j == width - 1 {
                    visible_count += 1;
                    continue;
                }
                visible_count += self.check_visible(i, j)
            }
        }
        visible_count
    }

    fn check_visible(&self, y: usize, x: usize) -> usize {
        let height = self.trees.len();
        let width = self.trees[0].len();
        let mut visible = true;
        for i in 0..y {
            if self.trees[i][x] >= self.trees[y][x] {
                visible = false;
                break;
            };
        }
        if visible {
            return 1;
        }
        visible = true;
        for i in y + 1..height {
            if self.trees[i][x] >= self.trees[y][x] {
                visible = false;
                break;
            }
        }
        if visible {
            return 1;
        }
        visible = true;
        for i in 0..x {
            if self.trees[y][i] >= self.trees[y][x] {
                visible = false;
                break;
            }
        }
        if visible {
            return 1;
        }
        visible = true;
        for i in x + 1..width {
            if self.trees[y][i] >= self.trees[y][x] {
                visible = false;
                break;
            }
        }
        if visible {
            return 1;
        }
        0
    }

    fn get_scenic_score(&self, x: usize, y: usize) -> usize {
        let mut results = vec![0, 0, 0, 0];
        let height = self.trees.len();
        let width = self.trees[0].len();
        for i in (0..x).rev() {
            results[0] += 1;
            if self.trees[y][i] >= self.trees[y][x] {
                break;
            }
        }
        for i in x + 1..width {
            results[1] += 1;
            if self.trees[y][i] >= self.trees[y][x] {
                break;
            }
        }
        for i in (0..y).rev() {
            results[2] += 1;
            if self.trees[i][x] >= self.trees[y][x] {
                break;
            }
        }
        for i in y + 1..height {
            results[3] += 1;
            if self.trees[i][x] >= self.trees[y][x] {
                break;
            }
        }
        results.iter().fold(1, |acc, &e| acc * e)
    }

    fn get_max_scenic_score(&self) -> usize {
        let mut max_score = 0;
        let height = self.trees.len();
        let width = self.trees[0].len();
        for y in 0..height {
            for x in 0..width {
                let score = self.get_scenic_score(x, y);
                max_score = cmp::max(max_score, score);
            }
        }
        max_score
    }
}

fn main() {
    let content = fs::read_to_string("input.txt").expect("File not found");
    let t = TreePatch::load(&content);
    part1(&t);
    part2(&t);
}

fn part1(t: &TreePatch) {
    let count = t.get_visible_count();
    println!("{}", count);
}

fn part2(t: &TreePatch) {
    let result = t.get_max_scenic_score();
    println!("{}", result);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_from_commands() -> Result<(), String> {
        let s = fs::read_to_string("test_input.txt").expect("File not found");

        let t = TreePatch::load(&s);
        assert_eq!(t.get_visible_count(), 21);
        Ok(())
    }

    #[test]
    fn test_scenic_score() -> Result<(), String> {
        let s = fs::read_to_string("test_input.txt").expect("File not found");

        let t = TreePatch::load(&s);
        assert_eq!(t.get_scenic_score(2, 1), 4);
        assert_eq!(t.get_scenic_score(2, 3), 8);
        Ok(())
    }

    #[test]
    fn test_get_max_scenic_score() -> Result<(), String> {
        let s = fs::read_to_string("test_input.txt").expect("File not found");

        let t = TreePatch::load(&s);
        assert_eq!(t.get_max_scenic_score(), 8);
        Ok(())
    }
}
