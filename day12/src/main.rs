use std::{
    collections::{HashMap, HashSet},
    fs,
};

#[derive(Debug)]
struct Dijkstra {
    grid: Vec<Vec<u32>>,
    width: u32,
    height: u32,
    dist: HashMap<u32, u32>,
    prev: HashMap<u32, u32>,
    q: HashSet<u32>,
    start_index: (u32, u32),
    end_index: (u32, u32),
}

impl Dijkstra {
    fn new(s: &str) -> Self {
        let grid: Vec<Vec<_>> = s
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| match c {
                        'S' => 0,
                        'E' => 25,
                        _ => c as u32 - 'a' as u32,
                    })
                    .collect()
            })
            .collect();
        let width = grid[0].len() as u32;
        let height = grid.len() as u32;
        let mut start_index = (0, 0);
        let mut end_index = (0, 0);
        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == 'S' {
                    start_index = (x as u32, y as u32);
                } else if c == 'E' {
                    end_index = (x as u32, y as u32);
                }
            }
        }

        Self {
            grid,
            width,
            height,
            dist: HashMap::new(),
            prev: HashMap::new(),
            q: HashSet::new(),
            start_index,
            end_index,
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
        for y in 0..self.height {
            for x in 0..self.width {
                let index = self.get_index_value(x, y);
                self.dist.insert(index, u32::MAX);
                self.prev.insert(index, 0);
                self.q.insert(index);
            }
        }
        let (start_x, start_y) = self.start_index;
        let start_index = self.get_index_value(start_x, start_y);
        self.dist.insert(start_index, 0);
    }

    fn run(&mut self) -> u32 {
        self.initialise();
        while !self.q.is_empty() {
            let next_index = self.get_next_vertex();

            if self.dist[&next_index] == u32::MAX {
                let (end_x, end_y) = self.end_index;
                return self.dist[&self.get_index_value(end_x, end_y)];
            }
            self.q.remove(&next_index);
            let neighbours = self.get_valid_neighbours(next_index);
            for neighbour in neighbours.iter() {
                let new_dist = self.dist[&next_index] + 1;
                if new_dist < self.dist[&neighbour] {
                    self.dist.insert(*neighbour, new_dist);
                    self.prev.insert(*neighbour, new_dist);
                }
            }
        }
        let (end_x, end_y) = self.end_index;
        self.dist[&self.get_index_value(end_x, end_y)]
    }

    fn get_valid_neighbours(&self, index: u32) -> Vec<u32> {
        let (x, y) = self.get_coords_from_value(index);
        let value = self.grid[y as usize][x as usize];
        let to_check: Vec<(i32, i32)> = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
        let mut result: Vec<u32> = Vec::new();

        for (i, j) in to_check.iter() {
            let n_x = x as i32 + i;
            let n_y = y as i32 + j;
            if n_x < 0 || n_x as u32 >= self.width || n_y < 0 || n_y as u32 >= self.height {
                continue;
            }
            if value + 1 < self.grid[n_y as usize][n_x as usize] {
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
        let mut index = u32::MAX;
        let mut min = u32::MAX;
        for i in self.q.iter() {
            if index == u32::MAX {
                index = *i;
            }
            if self.dist[i] < min {
                index = *i;
                min = self.dist[i];
            }
        }
        index as u32
    }
}

fn main() {
    let s = fs::read_to_string("input.txt").expect("File not found");
    part1(&s);
}

fn part1(s: &str) {
    let mut d = Dijkstra::new(s);
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
        assert_eq!(d.grid[0][0], 0);
        assert_eq!(d.grid[2][4], 25);
        Ok(())
    }

    #[test]
    fn test_get_start_index() -> Result<(), String> {
        let s = fs::read_to_string("test_input.txt").expect("File not found");

        let d = Dijkstra::new(&s);
        assert_eq!(d.start_index, (0, 0));
        assert_eq!(d.end_index, (5, 2));

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
}
