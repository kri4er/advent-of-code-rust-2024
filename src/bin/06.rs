use std::collections::{HashMap, HashSet};

use nom::AsChar;

advent_of_code::solution!(6);

#[derive(Debug, PartialEq, Hash, Eq, Clone, Copy)]
pub struct Point(i32, i32);

impl Point {
    pub fn add(self, other:&Point) -> Point {
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

fn turn(current_pos:usize) -> usize {
    if current_pos == 3 { 0 } else { current_pos + 1 }
}


pub fn part_one(input: &str) -> Option<u32> {
    let m = input.lines()
        .collect::<Vec<_>>();

    let mut m:HashMap<Point, char> = vec_to_hashmap(&m);

    let starting_pos: Point = m
        .iter()
        .find_map(|(&key, &val)| if val == '^' { Some(key) } else { None })
        .unwrap();

    let mut pos = Some((starting_pos.clone(), 0));
    while pos.is_some() {
        pos = next(pos.unwrap(), &m, &Point(-1,-1));
        if pos.is_some() {
            m.insert(pos.unwrap().0, 'X');
        }
    }
    let result = m.values().filter(|v| *v == &'X'.as_char()).count();

    //since we have skipped point 0 we should add +1
    Some((result + 1) as u32)
}

fn next(iter:(Point, usize), m:&HashMap<Point, char>, obstacle_loc: &Point)
    -> Option<(Point, usize)> {
    let(loc, dir) = iter;
    let next_pos = loc.add(&DIRS[dir]);
    let next_loc = m.get(&next_pos).unwrap_or(&'E');
    if &next_pos == obstacle_loc || next_loc == &'#' {
        return Some((loc, turn(dir)));
    } else if next_loc == &'E' {
        return None;
    }
    return Some((next_pos, dir));
}

fn is_looped(m:&HashMap<Point, char>, start_pos:Point, obstacle_loc: &Point) -> bool {
    let mut slow_pos = Some((start_pos.clone(), 0));

    let mut fast_pos = Some((start_pos.clone(), 0));
    while fast_pos.is_some() && next(fast_pos.unwrap(), &m, obstacle_loc).is_some() {
        slow_pos = next(slow_pos.unwrap(), m, obstacle_loc);

        fast_pos = next(fast_pos.unwrap(), m, obstacle_loc);
        fast_pos = next(fast_pos.unwrap(), m, obstacle_loc);
        if slow_pos == fast_pos {
            return true;
        }
    }
    return false;
}

pub fn part_two(input: &str) -> Option<u32> {
    let m = input.lines()
        .collect::<Vec<_>>();

    let m:HashMap<Point, char> = vec_to_hashmap(&m);

    let starting_pos: Point = m
        .iter()
        .find_map(|(&key, &val)| if val == '^' { Some(key) } else { None })
        .unwrap();

    let starting_point = starting_pos.clone();
    let mut path:HashSet<Point> = HashSet::new();
    let mut pos = Some((starting_pos.clone(), 0));
    while pos.is_some() {
        pos = next(pos.unwrap(), &m, &Point(-1,-1));
        if pos.is_some() {
            path.insert(pos.unwrap().0);
        }
    }
    let result = path.iter()
        .filter(|&obstacle_loc| {
            obstacle_loc != &starting_point && is_looped(&m, starting_point, obstacle_loc)
        })
        .count();


    Some(result as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(42));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }
}
