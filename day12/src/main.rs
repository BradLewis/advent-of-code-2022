use firestorm::profile_method;
use std::{collections::HashSet, fs};

#[derive(Debug)]
struct Dijkstra {
    height_grid: Vec<Vec<u32>>,
    grid: Vec<Vec<char>>,
    width: u32,
    height: u32,
    dist: Vec<u32>,
    q: HashSet<u32>,
    start_index: (u32, u32),
    end_value: char,
    reverse: bool,
}

impl Dijkstra {
    fn new(s: &str) -> Self {
        let grid: Vec<Vec<_>> = s.lines().map(|l| l.chars().map(|c| c).collect()).collect();
        let height_grid: Vec<Vec<_>> = grid
            .iter()
            .map(|r| {
                r.iter()
                    .map(|c| match c {
                        'S' => 0,
                        'E' => 25,
                        _ => *c as u32 - 'a' as u32,
                    })
                    .collect()
            })
            .collect();

        let width = height_grid[0].len() as u32;
        let height = height_grid.len() as u32;
        let mut start_index = (0, 0);
        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == 'S' {
                    start_index = (x as u32, y as u32);
                }
            }
        }

        Self {
            height_grid,
            grid,
            width,
            height,
            dist: vec![u32::MAX; (width * height) as usize],
            q: HashSet::new(),
            start_index,
            end_value: 'E',
            reverse: false,
        }
    }

    fn get_index_value(&self, x: u32, y: u32) -> u32 {
        y * self.width + x
    }

    fn get_coords_from_value(&self, value: u32) -> (u32, u32) {
        let x = value % self.width;
        let y = (value - x) / self.width;
        (x, y)
    }

    fn initialise(&mut self) {
        profile_method!(initialise);
        for y in 0..self.height {
            for x in 0..self.width {
                let index = self.get_index_value(x, y);
                self.q.insert(index);
            }
        }
        let (start_x, start_y) = self.start_index;
        let start_index = self.get_index_value(start_x, start_y);
        self.dist[start_index as usize] = 0;
    }

    fn run(&mut self) -> u32 {
        profile_method!(run);
        self.initialise();
        while !self.q.is_empty() {
            let next_index = self.get_next_vertex();

            let (x, y) = self.get_coords_from_value(next_index);
            if self.grid[y as usize][x as usize] == self.end_value {
                return self.dist[next_index as usize];
            }
            self.q.remove(&next_index);
            let neighbours = match self.reverse {
                false => self.get_valid_neighbours(next_index),
                true => self.get_valid_neighbours_reverse(next_index),
            };
            for neighbour in neighbours.iter() {
                let new_dist = self.dist[next_index as usize] + 1;
                if new_dist < self.dist[*neighbour as usize] {
                    self.dist[*neighbour as usize] = new_dist;
                }
            }
        }
        unreachable!();
    }

    fn get_valid_neighbours(&self, index: u32) -> Vec<u32> {
        profile_method!(get_valid_neighbours);
        let (x, y) = self.get_coords_from_value(index);
        let value = self.height_grid[y as usize][x as usize];
        let to_check: Vec<(i32, i32)> = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
        let mut result: Vec<u32> = Vec::new();

        for (i, j) in to_check.iter() {
            let n_x = x as i32 + i;
            let n_y = y as i32 + j;
            if n_x < 0 || n_x as u32 >= self.width || n_y < 0 || n_y as u32 >= self.height {
                continue;
            }
            if value + 1 < self.height_grid[n_y as usize][n_x as usize] {
                continue;
            }
            let new_index = self.get_index_value(n_x as u32, n_y as u32);
            if self.q.contains(&new_index) {
                result.push(new_index);
            }
        }
        result
    }

    fn get_valid_neighbours_reverse(&self, index: u32) -> Vec<u32> {
        profile_method!(get_valid_neighbours_reverse);
        let (x, y) = self.get_coords_from_value(index);
        let value = self.height_grid[y as usize][x as usize];
        let to_check: Vec<(i32, i32)> = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
        let mut result: Vec<u32> = Vec::new();

        for (i, j) in to_check.iter() {
            let n_x = x as i32 + i;
            let n_y = y as i32 + j;
            if n_x < 0 || n_x as u32 >= self.width || n_y < 0 || n_y as u32 >= self.height {
                continue;
            }
            if value - 1 > self.height_grid[n_y as usize][n_x as usize] {
                continue;
            }
            let new_index = self.get_index_value(n_x as u32, n_y as u32);
            if self.q.contains(&new_index) {
                result.push(new_index);
            }
        }
        result
    }

    fn get_next_vertex(&self) -> u32 {
        profile_method!(get_next_vertex);
        let (min_element, _) = self
            .q
            .iter()
            .map(|i| (*i, self.dist[*i as usize]))
            .min_by_key(|(_, d)| *d)
            .unwrap();
        min_element
    }

    fn get_index_of(&self, c: char) -> (u32, u32) {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.grid[y as usize][x as usize] == c {
                    return (x, y);
                }
            }
        }
        unreachable!();
    }
}

fn main() {
    firestorm::bench("./flames/part1/", part1).unwrap();
    firestorm::bench("./flames/part2/", part2).unwrap();
}

fn part1() {
    let s = fs::read_to_string("input.txt").expect("File not found");
    let mut d = Dijkstra::new(&s);
    let result = d.run();
    println!("{}", result);
}

fn part2() {
    let s = fs::read_to_string("input.txt").expect("File not found");
    let mut d = Dijkstra::new(&s);
    d.end_value = 'a';
    d.start_index = d.get_index_of('E');
    d.reverse = true;

    let result = d.run();
    println!("{}", result);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_load_grid() -> Result<(), String> {
        let s = fs::read_to_string("test_input.txt").expect("File not found");

        let d = Dijkstra::new(&s);
        assert_eq!(d.height_grid[0][0], 0);
        assert_eq!(d.height_grid[2][4], 25);
        Ok(())
    }

    #[test]
    fn test_get_start_index() -> Result<(), String> {
        let s = fs::read_to_string("test_input.txt").expect("File not found");

        let d = Dijkstra::new(&s);
        assert_eq!(d.start_index, (0, 0));

        Ok(())
    }

    #[test]
    fn test_dijkstra_algorithm() -> Result<(), String> {
        let s = fs::read_to_string("test_input.txt").expect("File not found");

        let mut d = Dijkstra::new(&s);
        let result = d.run();
        assert_eq!(result, 31);
        Ok(())
    }

    #[test]
    fn test_get_index() -> Result<(), String> {
        let s = fs::read_to_string("test_input.txt").expect("File not found");

        let d = Dijkstra::new(&s);
        assert_eq!(d.get_index_value(1, 3), 8 * 3 + 1);
        Ok(())
    }

    #[test]
    fn test_get_coords() -> Result<(), String> {
        let s = fs::read_to_string("test_input.txt").expect("File not found");

        let d = Dijkstra::new(&s);
        assert_eq!(d.get_coords_from_value(25), (1, 3));
        assert_eq!(d.get_coords_from_value(0), (0, 0));
        Ok(())
    }

    #[test]
    fn test_get_next_vertex() -> Result<(), String> {
        let s = fs::read_to_string("test_input.txt").expect("File not found");

        let mut d = Dijkstra::new(&s);
        d.initialise();

        assert_eq!(d.get_next_vertex(), 0);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<(), String> {
        let s = fs::read_to_string("test_input.txt").expect("File not found");

        let mut d = Dijkstra::new(&s);
        let start_index = d.get_index_of('E');
        d.start_index = start_index;
        d.end_value = 'a';
        d.reverse = true;
        let result = d.run();
        assert_eq!(result, 29);
        Ok(())
    }
}
