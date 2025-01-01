use std::{collections::{HashMap, HashSet}, ops::{Add, Sub}};

advent_of_code::solution!(12);

use advent_of_code::util::union_find::UnionFind;

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub enum Side {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Hash, Eq, Clone, Copy)]
pub struct Point(i32, i32);


impl Point {
    pub fn add(self, other:&Point) -> Point {
        Point(self.0 + other.0, self.1 + other.1)
    }
    pub fn to_idx(self, width:usize) -> usize {
        self.1 as usize * width + self.0 as usize
    }

    pub fn neighbours(self)->[Point;4] {
        ORTHOGONAL.map(|dir| self + dir)
    }
    pub fn neighbour_on_side(self, side:Side)->Point {
        match side {
            Side::Up => Point(self.0, self.1-1),
            Side::Down => Point(self.0, self.1+1),
            Side::Left => Point(self.0-1, self.1),
            Side::Right => Point(self.0+1, self.1),
        }
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

static ORTHOGONAL: &'static [Point; 4] =
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

fn perimeter(plots:&HashSet<Point>) -> usize {
    let mut visited: HashSet<Point> = HashSet::new();

    let mut perimeter = 0;

    for c in plots.iter() {
        if visited.contains(c) {
            continue;
        }

        visited.insert(*c);

        for n in c.neighbours() {
            if !plots.contains(&n) {
                perimeter += 1;
            }
        }
        // count out of set neighbours
    }

    perimeter
}

pub fn part_one(input: &str) -> Option<u32> {
    let m = input.lines()
        .collect::<Vec<_>>();
    let height = m.len();
    let width = m[0].len();
    let mut uf = (0..width*height).collect::<UnionFind<_>>();
    let m:HashMap<Point, char> = vec_to_hashmap(&m);

    for p in m.iter() {
        let cur = p.0.clone();
        for next in cur.neighbours() {
            if m.get(&cur) == m.get(&next) {
                let cur_id = cur.to_idx(width);
                let next_id = next.to_idx(width);
                uf.union(&cur_id, &next_id);
            }
        }
    }
    let mut regions:HashMap<usize, HashSet<Point>> = HashMap::new();

    for p in m.iter() {
        let cur = p.0.clone();
        regions
            .entry(uf.find(&cur.to_idx(width)).unwrap())
            .or_default()
            .insert(cur.clone());
    }

    let mut result = 0;
    for region in regions.iter() {
        result += region.1.len() * perimeter(region.1);
    }
    Some(result as u32)
}

fn sides(plots:&HashSet<Point>) -> u32 {
        let mut side_set: HashSet<(Point, Side)> = HashSet::new();

        for plot in plots {
            for s in [Side::Up, Side::Down, Side::Left, Side::Right] {
                if !plots.contains(&plot.neighbour_on_side(s)) {
                    side_set.insert((*plot, s));
                }
            }
        }

        let mut grouped_sides: HashMap<(Point,Side), i32> = HashMap::new();
        for side_entry in side_set.iter() {
            let mut current = side_entry.0;

            match side_entry.1 {
                Side::Up|Side::Down => {
                    let mut to_the_left = current.neighbour_on_side(Side::Left);
                    while side_set.contains(&(to_the_left, side_entry.1)) {
                        current = to_the_left;
                        to_the_left = current.neighbour_on_side(Side::Left);
                    }

                    let key = (current, side_entry.1);
                    if grouped_sides.contains_key(&key) {
                        continue;
                    }

                    let mut side_length = 1;
                    let mut to_the_right = current.neighbour_on_side(Side::Right);

                    while side_set.contains(&(to_the_right, side_entry.1)) {
                        current = to_the_right;
                        to_the_right = current.neighbour_on_side(Side::Right);
                        side_length += 1;
                    }

                    grouped_sides.insert(key, side_length);
                }
                Side::Left|Side::Right => {
                    let mut below = current.neighbour_on_side(Side::Down);
                    while side_set.contains(&(below, side_entry.1)) {
                        current = below;
                        below = current.neighbour_on_side(Side::Down);
                    }

                    let key = (current, side_entry.1);

                    if grouped_sides.contains_key(&key) {
                        continue;
                    }

                    let mut side_length = 1;
                    let mut above = current.neighbour_on_side(Side::Up);
                    while side_set.contains(&(above, side_entry.1)) {
                        current = above;
                        above = current.neighbour_on_side(Side::Up);
                        side_length += 1;
                    }

                    grouped_sides.insert(key, side_length);
                }
            }
        }

        grouped_sides.len() as u32
    }

pub fn part_two(input: &str) -> Option<u32> {
    let m = input.lines()
        .collect::<Vec<_>>();
    let height = m.len();
    let width = m[0].len();
    let mut uf = (0..width*height).collect::<UnionFind<_>>();
    let m:HashMap<Point, char> = vec_to_hashmap(&m);

    for p in m.iter() {
        let cur = p.0.clone();
        for next in cur.neighbours() {
            if m.get(&cur) == m.get(&next) {
                let cur_id = cur.to_idx(width);
                let next_id = next.to_idx(width);
                uf.union(&cur_id, &next_id);
            }
        }
    }
    let mut regions:HashMap<usize, HashSet<Point>> = HashMap::new();

    for p in m.iter() {
        let cur = p.0.clone();
        regions
            .entry(uf.find(&cur.to_idx(width)).unwrap())
            .or_default()
            .insert(cur.clone());
    }

    let mut result = 0;
    for region in regions.iter() {
        result += region.1.len() * sides(region.1) as usize;
    }
    Some(result as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}
