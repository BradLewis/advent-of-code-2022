use crate::robots::Robot;

#[derive(Debug)]
pub struct Blueprint {
    pub robots: Vec<Robot>,
}

impl Blueprint {
    pub fn create(s: &str) -> Self {
        let (_, stripped) = s.split_once(':').unwrap();
        let robots: Vec<_> = stripped.split(". ").map(Robot::new).collect();
        Self { robots }
    }
}

mod tests {
    use super::Blueprint;

    #[test]
    fn test_blueprint_creation() {
        let s = String::from("Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.");
        let blueprint = Blueprint::create(&s);

        assert_eq!(blueprint.robots.len(), 4);
    }
}
