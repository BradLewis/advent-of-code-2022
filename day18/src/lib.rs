#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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
}

#[derive(Debug)]
struct Node {
    position: Position,
    connections: Vec<Position>,
}

#[derive(Debug)]
pub struct Graph {
    nodes: Vec<Node>,
}

impl Graph {
    pub fn from_string(s: &str) -> Self {
        let positions: Vec<_> = s
            .lines()
            .map(|l| {
                let p: Vec<_> = l.split(",").map(|x| x.parse::<isize>().unwrap()).collect();
                Position::new(p[0], p[1], p[2])
            })
            .collect();

        Graph::generate(&positions)
    }

    fn generate(positions: &[Position]) -> Self {
        let mut nodes: Vec<Node> = Vec::new();
        for position in positions.iter() {
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
        Self { nodes }
    }

    pub fn surface_area(&self) -> usize {
        let mut total = 0;
        for node in self.nodes.iter() {
            total += 6 - node.connections.len();
        }
        total
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
        let graph = Graph::generate(&p);

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
        Ok(())
    }

    #[test]
    fn test_graph_surface_area() -> Result<(), String> {
        let p = vec![Position::new(1, 1, 1), Position::new(2, 1, 1)];
        let graph = Graph::generate(&p);
        assert_eq!(graph.surface_area(), 10);

        let s = fs::read_to_string("test_input.txt").expect("File not found");
        let graph = Graph::from_string(&s);
        assert_eq!(graph.surface_area(), 64);
        Ok(())
    }
}
