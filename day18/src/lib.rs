use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SearchResult {
    Searching,
    Trapped,
    Free,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Position {
    x: isize,
    y: isize,
    z: isize,
}

impl Position {
    fn new(x: isize, y: isize, z: isize) -> Self {
        Self { x, y, z }
    }

    fn distance(&self, position: Position) -> isize {
        (self.x - position.x).abs() + (self.y - position.y).abs() + (self.z - position.z).abs()
    }

    fn neighbours(&self) -> Vec<Position> {
        vec![
            Position::new(self.x - 1, self.y, self.z),
            Position::new(self.x + 1, self.y, self.z),
            Position::new(self.x, self.y - 1, self.z),
            Position::new(self.x, self.y + 1, self.z),
            Position::new(self.x, self.y, self.z - 1),
            Position::new(self.x, self.y, self.z + 1),
        ]
    }
}

#[derive(Debug)]
struct Node {
    position: Position,
    connections: Vec<Position>,
}

impl Node {
    fn missing_connections(&self) -> Vec<Position> {
        let to_check = self.position.neighbours();
        to_check
            .into_iter()
            .filter(|p| !self.connections.contains(p))
            .collect()
    }
}

#[derive(Debug)]
pub struct Graph {
    nodes: Vec<Node>,
    positions: HashSet<Position>,
    max_position: Position,
    min_position: Position,
}

impl Graph {
    pub fn from_string(s: &str) -> Self {
        let positions: Vec<_> = s
            .lines()
            .map(|l| {
                let p: Vec<_> = l.split(',').map(|x| x.parse::<isize>().unwrap()).collect();
                Position::new(p[0], p[1], p[2])
            })
            .collect();

        Graph::generate(positions)
    }

    fn generate(positions: Vec<Position>) -> Self {
        let mut max_position = Position::new(0, 0, 0);
        let mut nodes: Vec<Node> = Vec::new();
        for position in positions.iter() {
            max_position.x = max_position.x.max(position.x);
            max_position.y = max_position.y.max(position.y);
            max_position.z = max_position.z.max(position.z);
            let mut connections: Vec<Position> = Vec::new();
            for node in nodes.iter_mut() {
                if node.position.distance(*position) == 1 {
                    node.connections.push(*position);
                    connections.push(node.position);
                }
            }
            nodes.push(Node {
                position: *position,
                connections,
            })
        }
        Self {
            nodes,
            positions: HashSet::from_iter(positions),
            max_position,
            min_position: Position::new(0, 0, 0),
        }
    }

    pub fn surface_area(&self, include_trapped: bool) -> usize {
        let mut total = 0;
        let mut cache = HashMap::new();
        for node in self.nodes.iter() {
            let mut num_neighbours = 0;
            let neighbours = node.position.neighbours();
            for neighbour in neighbours.iter() {
                if node.connections.contains(neighbour) {
                    num_neighbours += 1;
                    continue;
                }
                if !include_trapped && self.check_trapped(*neighbour, &mut cache) {
                    num_neighbours += 1;
                }
            }
            total += 6 - num_neighbours;
        }
        total
    }

    fn check_out_of_bounds(&self, position: &Position) -> bool {
        if position.x > self.max_position.x {
            return true;
        } else if position.x < self.min_position.x {
            return true;
        } else if position.y > self.max_position.y {
            return true;
        } else if position.y < self.min_position.y {
            return true;
        } else if position.z > self.max_position.z {
            return true;
        } else if position.z < self.min_position.z {
            return true;
        }
        false
    }

    fn check_trapped(
        &self,
        position: Position,
        cache: &mut HashMap<Position, SearchResult>,
    ) -> bool {
        // println!("{:?}", position);
        if cache.contains_key(&position) {
            let result = cache[&position];
            match result {
                SearchResult::Free => return false,
                SearchResult::Trapped => return true,
                _ => (),
            }
        }
        cache.insert(position, SearchResult::Searching);
        let mut skip_count = 0;
        for neighbour in position.neighbours().iter() {
            if self.positions.contains(neighbour) {
                skip_count += 1;
                continue;
            }
            if cache.contains_key(neighbour) && cache[neighbour] == SearchResult::Searching {
                skip_count += 1;
                continue;
            }
            if self.check_out_of_bounds(neighbour) {
                cache.insert(position, SearchResult::Free);
                return false;
            }
            if !self.check_trapped(*neighbour, cache) {
                cache.insert(position, SearchResult::Free);
                return false;
            }
        }
        if skip_count != 6 {
            cache.insert(position, SearchResult::Trapped);
        } else {
            cache.remove(&position);
        }
        true
    }
}

#[cfg(test)]
mod tests {

    use std::fs;

    use super::*;

    #[test]
    fn test_position_distance() -> Result<(), String> {
        let p1 = Position::new(1, 1, 1);
        let p2 = Position::new(2, 1, 1);
        assert_eq!(p1.distance(p2), 1);
        Ok(())
    }

    #[test]
    fn test_graph_load() -> Result<(), String> {
        let p = vec![Position::new(1, 1, 1), Position::new(2, 1, 1)];
        let graph = Graph::generate(p);

        assert_eq!(graph.nodes.len(), 2);
        assert_eq!(graph.nodes[0].connections.len(), 1);
        assert_eq!(graph.nodes[0].connections[0], Position::new(2, 1, 1));
        assert_eq!(graph.nodes[1].connections.len(), 1);
        assert_eq!(graph.nodes[1].connections[0], Position::new(1, 1, 1));
        Ok(())
    }

    #[test]
    fn test_graph_from_str() -> Result<(), String> {
        let s = fs::read_to_string("test_input.txt").expect("File not found");
        let graph = Graph::from_string(&s);

        assert_eq!(graph.nodes.len(), 13);
        assert_eq!(graph.max_position, Position::new(3, 3, 6));
        Ok(())
    }

    #[test]
    fn test_graph_surface_area() -> Result<(), String> {
        let p = vec![Position::new(1, 1, 1), Position::new(2, 1, 1)];
        let graph = Graph::generate(p);
        assert_eq!(graph.surface_area(true), 10);

        let s = fs::read_to_string("test_input.txt").expect("File not found");
        let graph = Graph::from_string(&s);
        assert_eq!(graph.surface_area(true), 64);
        Ok(())
    }

    #[test]
    fn test_node_missing_connections() -> Result<(), String> {
        let p = vec![Position::new(1, 1, 1), Position::new(2, 1, 1)];
        let graph = Graph::generate(p);
        let n = &graph.nodes[0];
        let mc = n.missing_connections();

        assert_eq!(mc.len(), 5);
        Ok(())
    }

    #[test]
    fn test_graph_node_trapped() -> Result<(), String> {
        let s = fs::read_to_string("test_input.txt").expect("File not found");
        let graph = Graph::from_string(&s);

        let result = graph.check_trapped(Position::new(2, 2, 5), &mut HashMap::new());
        assert!(result);
        assert!(!graph.check_trapped(Position::new(1, 1, 5), &mut HashMap::new()));
        Ok(())
    }

    #[test]
    fn test_graph_surface_area_exclude_trapped() -> Result<(), String> {
        let p = vec![Position::new(1, 1, 1), Position::new(2, 1, 1)];
        let graph = Graph::generate(p);
        assert_eq!(graph.surface_area(false), 10);

        let s = fs::read_to_string("test_input.txt").expect("File not found");
        let graph = Graph::from_string(&s);
        assert_eq!(graph.surface_area(false), 58);
        Ok(())
    }
}
