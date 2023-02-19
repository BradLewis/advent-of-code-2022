#[macro_use]
extern crate lazy_static;

use std::{cmp, collections::BTreeSet};

use firestorm::{profile_fn, profile_method};
use regex::Regex;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Point {
    x: isize,
    y: isize,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Range {
    start: isize,
    end: isize,
}

impl Range {
    fn new(start: isize, end: isize) -> Self {
        Self { start, end }
    }
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

    fn get_overlap(&self, y: isize) -> Option<Range> {
        profile_method!(get_overlap);
        let common = self.closest_beacon_distance - (y - self.position.y).abs();
        if common < 0 {
            return None;
        }
        let x_start = self.position.x - common;
        let x_end = self.position.x + common;
        Some(Range::new(x_start, x_end))
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

fn merge_ranges(arr: &mut Vec<Range>) -> Vec<Range> {
    profile_fn!(merge_ranges);
    arr.sort_by(|a, b| a.start.cmp(&b.start));
    let mut result: Vec<Range> = Vec::new();
    result.push(arr[0]);

    (1..arr.len()).for_each(|i| {
        let current = arr[i];
        let j: usize = result.len() - 1;

        if current.start >= result[j].start && current.start <= result[j].end + 1 {
            result[j].end = cmp::max(current.end, result[j].end);
        } else {
            result.push(current);
        }
    });
    result
}

pub fn part1(s: &str, row: isize) -> isize {
    profile_fn!(part1);
    let mut beacons_on_row: BTreeSet<isize> = BTreeSet::new();

    let mut overlaps: Vec<Range> = s
        .lines()
        .filter_map(|l| {
            let sensor = Sensor::from_string(l);
            if sensor.closest_beacon.y == row {
                beacons_on_row.insert(sensor.closest_beacon.x);
            }
            sensor.get_overlap(row)
        })
        .collect();
    let ranges = merge_ranges(&mut overlaps);
    let mut count = 0;
    for range in ranges.iter() {
        count += range.end - range.start + 1;
    }
    count - beacons_on_row.len() as isize
}

pub fn part2(s: &str, max_row: usize) -> isize {
    profile_fn!(part2);
    let sensors: Vec<_> = s.lines().map(Sensor::from_string).collect();
    for i in 0..max_row {
        let mut overlaps: Vec<_> = sensors
            .iter()
            .filter_map(|s| s.get_overlap(i as isize))
            .collect();
        let ranges = merge_ranges(&mut overlaps);
        if ranges.len() != 1 {
            return 4000000 * (ranges[0].end + 1) + (i as isize);
        }
    }
    unreachable!();
}

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

        let result = Range::new(-2, 4);
        assert_eq!(overlap, Some(result));
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

    #[test]
    fn test_part2() -> Result<(), String> {
        let s = fs::read_to_string("test_input.txt").expect("File not found");
        let result = part2(&s, 20);

        assert_eq!(result, 56000011);
        Ok(())
    }

    #[test]
    fn test_merge_ranges() -> Result<(), String> {
        let result = merge_ranges(&mut vec![Range::new(1, 4), Range::new(2, 10)]);
        assert_eq!(result, vec![Range::new(1, 10)]);

        let result = merge_ranges(&mut vec![Range::new(1, 4), Range::new(6, 10)]);
        assert_eq!(result, vec![Range::new(1, 4), Range::new(6, 10)]);

        let result = merge_ranges(&mut vec![Range::new(1, 4), Range::new(5, 10)]);
        assert_eq!(result, vec![Range::new(1, 10)]);

        let result = merge_ranges(&mut vec![
            Range::new(12, 12),
            Range::new(2, 14),
            Range::new(-2, 2),
            Range::new(16, 24),
            Range::new(14, 18),
        ]);
        assert_eq!(result, vec![Range::new(-2, 24)]);
        Ok(())
    }
}
