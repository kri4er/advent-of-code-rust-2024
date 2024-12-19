use std::{collections::{HashMap, HashSet}, process::Output};
use core::ops::{Sub, Add, Mul};

use nom::AsChar;

advent_of_code::solution!(8);

#[derive(Debug, PartialEq, Hash, Eq, Clone, Copy)]
pub struct Point(i32, i32);

impl Point {
    pub fn add(self, other:&Point) -> Point {
        Point(self.0 + other.0, self.1 + other.1)
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, other:Point) -> Point {
        Point(self.0 - other.0, self.1 - other.1)
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, other:Point) -> Point {
        Point(self.0 + other.0, self.1 + other.1)
    }
}

static DIRS: &'static [Point; 4] =
&[  Point(-1, 0),//up
    Point(0, 1),//right
    Point(1, 0),//down
    Point(0, -1),//left
];


fn vec_to_hashmap(data: &Vec<&str>) -> HashMap<Point, char> {
    data.iter()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(move |(col, ch)|
                    (Point(row as i32, col as i32), ch)
                )
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let m = input.lines()
        .collect::<Vec<_>>();


    let m:HashMap<Point, char> = vec_to_hashmap(&m);
    let anthenas:Vec<Point> = m.iter()
        .filter(|&(_k, v)| *v != '.'.as_char())
        .map(|(k, _v)| k.clone())
        .collect();

    let mut antinodes = HashSet::<Point>::new();

    for i in 0..anthenas.len() {
        for j in i+1..anthenas.len() {
            let (ap0, ap1) = (anthenas[i], anthenas[j]);
            //skipe if different frequency
            if m.get(&ap0) != m.get(&ap1) {
                continue;
            }

            let v = ap0 - ap1; // vector from ps1 to ps0
            for potential_location in [ap0 + v, ap1 - v] {
                if m.contains_key(&potential_location) {
                    antinodes.insert(potential_location);
                }
            }
        }
    }

    Some(antinodes.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
