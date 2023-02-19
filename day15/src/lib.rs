#[macro_use]
extern crate lazy_static;

use std::{cmp, collections::BTreeSet};

use regex::Regex;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Point {
    x: isize,
    y: isize,
}

#[derive(Debug)]
struct Sensor {
    position: Point,
    closest_beacon: Point,
    closest_beacon_distance: isize,
}

impl Sensor {
    fn new(position: Point, closest_beacon: Point) -> Self {
        Self {
            position,
            closest_beacon,
            closest_beacon_distance: distance(position, closest_beacon),
        }
    }

    fn from_string(s: &str) -> Self {
        let numbers = str_strip_numbers(s);
        let position = Point {
            x: numbers[0],
            y: numbers[1],
        };
        let closest_beacon = Point {
            x: numbers[2],
            y: numbers[3],
        };
        Self::new(position, closest_beacon)
    }

    fn get_overlap(&self, y: isize) -> Vec<isize> {
        let common = self.closest_beacon_distance - (y - self.position.y).abs();
        let x_start = self.position.x - common;
        let x_end = self.position.x + common;
        (x_start..x_end + 1).collect()
    }
}

fn distance(p1: Point, p2: Point) -> isize {
    (p1.x - p2.x).abs() + (p1.y - p2.y).abs()
}

fn str_strip_numbers(s: &str) -> Vec<isize> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"-?\d+").unwrap();
    }
    // iterate over all matches
    RE.find_iter(s)
        // try to parse the string matches as i64 (inferred from fn type signature)
        // and filter out the matches that can't be parsed (e.g. if there are too many digits to store in an i64).
        .filter_map(|digits| digits.as_str().parse().ok())
        // collect the results in to a Vec<i64> (inferred from fn type signature)
        .collect()
}

pub fn part1(s: &str, row: isize) -> usize {
    let mut max_x = isize::MIN;
    let mut min_x = isize::MAX;
    let mut beacons_on_row: BTreeSet<isize> = BTreeSet::new();

    let overlaps: BTreeSet<isize> = s
        .lines()
        .map(|l| {
            let sensor = Sensor::from_string(l);
            if sensor.closest_beacon.y == row {
                beacons_on_row.insert(sensor.closest_beacon.x);
            }
            max_x = cmp::max(max_x, cmp::max(sensor.closest_beacon.x, sensor.position.x));
            min_x = cmp::min(min_x, cmp::min(sensor.closest_beacon.x, sensor.position.x));
            sensor.get_overlap(row)
        })
        .into_iter()
        .flatten()
        .collect();
    overlaps.len() - beacons_on_row.len()
}

pub fn part2(s: &str, max_row: usize) {}

#[cfg(test)]
mod tests {

    use std::fs;

    use super::*;

    #[test]
    fn test_distance() -> Result<(), String> {
        assert_eq!(distance(Point { x: 0, y: 0 }, Point { x: 0, y: 0 }), 0);
        assert_eq!(distance(Point { x: 10, y: 10 }, Point { x: -1, y: 0 }), 21);
        Ok(())
    }

    #[test]
    fn test_sensor_from_string() -> Result<(), String> {
        let sensor = Sensor::from_string("Sensor at x=2, y=18: closest beacon is at x=-2, y=15");
        assert_eq!(sensor.position, Point { x: 2, y: 18 });
        assert_eq!(sensor.closest_beacon, Point { x: -2, y: 15 });
        Ok(())
    }

    #[test]
    fn test_get_overlap_interval() -> Result<(), String> {
        let sensor = Sensor::new(Point { x: 1, y: 3 }, Point { x: 1, y: 8 });
        let overlap = sensor.get_overlap(1);

        let result: Vec<isize> = (-2..5).collect();
        assert_eq!(overlap, result);
        Ok(())
    }

    #[test]
    fn test_regex() -> Result<(), String> {
        let result = str_strip_numbers("Sensor at x=2, y=18: closest beacon is at x=-2, y=15");
        assert_eq!(result, vec![2, 18, -2, 15]);
        Ok(())
    }

    #[test]
    fn test_part1() -> Result<(), String> {
        let s = fs::read_to_string("test_input.txt").expect("File not found");
        let result = part1(&s, 10);

        assert_eq!(result, 26);
        Ok(())
    }
}
