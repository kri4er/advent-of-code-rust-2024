use std::{collections::{HashMap, HashSet}, ops::{Add, Sub}};

advent_of_code::solution!(10);

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


fn vec_to_hashmap(data: &Vec<&str>) -> HashMap<Point, usize> {
    data.iter()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(move |(col, ch)|
                    (Point(row as i32, col as i32), ch.to_digit(10).unwrap() as usize)
                )
        })
        .collect()
}

fn dfs(m:&HashMap<Point, usize>, visited:&mut HashMap<Point, Point>, point: Point, starting_point:Point) -> usize {
    let mut result = 0;

    for next in DIRS.map(|dir| point.clone() + dir.clone()) {
        if m.contains_key(&next) && m[&point] + 1 == m[&next]
            && visited.get(&next).unwrap_or(&Point(-1 as i32, -1 as i32)) != &starting_point {
            visited.insert(next.clone(), starting_point.clone());
            if m[&next] == 9 {
                result += 1;
            } else  {
                result += dfs(&m, visited, next, starting_point);
            }
        }
    }
    result
}

pub fn part_one(input: &str) -> Option<usize> {
    let m = input.lines()
        .collect::<Vec<_>>();


    let m:HashMap<Point, usize> = vec_to_hashmap(&m);
    let mut visited:HashMap<Point, Point> = HashMap::new();

    let result = m.iter().fold(0, |acc, p| {
        acc + if *p.1 == 0 {
            let res = dfs(&m, &mut visited, p.0.clone(), p.0.clone());
            res
        } else { 0 }
    });

    Some(result)

}

fn dfs2(m:&HashMap<Point, usize>, point: Point) -> usize {
    let mut result = 0;

    for next in DIRS.map(|dir| point.clone() + dir.clone()) {
        if m.contains_key(&next) && m[&point] + 1 == m[&next] {
            if m[&next] == 9 {
                result += 1;
            } else  {
                result += dfs2(&m, next);
            }
        }
    }
    result
}

pub fn part_two(input: &str) -> Option<usize> {
    let m = input.lines()
        .collect::<Vec<_>>();

    let m:HashMap<Point, usize> = vec_to_hashmap(&m);

    let result = m.iter().fold(0, |acc, p| {
        acc + if *p.1 == 0 {
            let res = dfs2(&m, p.0.clone());
            res
        } else { 0 }
    });

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
