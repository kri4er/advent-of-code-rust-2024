use std::{collections::HashSet, num::ParseIntError, ops::{Add, Mul, Sub}, str::FromStr};

use itertools::Itertools;

advent_of_code::solution!(14);

#[derive(Debug, PartialEq, Hash, Eq, Clone, Copy, PartialOrd, Ord)]
pub struct Point(i64, i64);


impl Point {
    pub fn add(self, other:&Point) -> Point {
        Point(self.0 + other.0, self.1 + other.1)
    }
    pub fn to_idx(self, width:usize) -> usize {
        self.1 as usize * width + self.0 as usize
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
impl Add<i64> for Point {
    type Output = Point;

    fn add(self, other:i64) -> Point {
        Point(self.0 + other, self.1 + other)
    }
}

impl Mul for Point {
    type Output = Point;

    fn mul(self, other:Point) -> Point {
        Point(self.0 * other.0, self.1 * other.1)
    }

}

impl Mul<usize> for Point {
    type Output = Point;

    fn mul(self, other:usize) -> Point {
        Point(self.0 * other as i64, self.1 * other as i64)
    }
}

impl Mul<i64> for Point {
    type Output = Point;

    fn mul(self, other:i64) -> Point {
        Point(self.0 * other, self.1 * other)
    }
}

#[derive(Debug, PartialEq)]
pub struct Robot {
    pos: Point,
    velo: Point,
}

impl Robot {
    fn travel(self:&Robot, time:usize) -> Point {
        self.pos + self.velo * time
    }
}

impl FromStr for Robot {
    type Err = std::num::ParseIntError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (pos, velo) = line.split_once(" ").unwrap();
        let pos = pos.strip_prefix("p=").unwrap().split_once(',').unwrap();
        let velo = velo.strip_prefix("v=").unwrap().split_once(',').unwrap();

        Ok(Robot {
            pos: Point(pos.0.parse().unwrap(), pos.1.parse().unwrap()),
            velo: Point(
                velo.0.parse::<i64>().unwrap().rem_euclid(101),
                velo.1.parse::<i64>().unwrap().rem_euclid(103)
                ),
        })
    }

}


#[derive(Debug, PartialEq)]
pub struct Game {
    width: usize,
    height: usize,
    robots: Vec<Robot>,
}

impl Game {
    fn wrap(&self, position: Point) -> Point {
        let (mut x, mut y) = (position.0 % self.width as i64, position.1 % self.height as i64);

        if x < 0 {
            x = self.width as i64 + x;
        }
        if y < 0 {
            y = self.height as i64 + y;
        }

        Point(x, y)
    }

    fn solve_linear(self:&Game, time: usize) -> usize {
        let v = (self.height / 2) as i64;
        let h = (self.width / 2) as i64;

        let res = self.robots.iter()
            .map(|robot| robot.travel(time))//new coordinates of each robot on a global map
            .map(|position| self.wrap(position))//coordinates wrapped into same canvas
            .fold([0; 4], |mut quads, position| {
                 if position.0 < h {
                     if position.1 < v {
                         quads[0] += 1;
                     }
                     if position.1 > v {
                         quads[2] += 1;
                     }
                 }
                 if position.0 > h {
                     if position.1 < v {
                         quads[1] += 1;
                     }
                     if position.1 > v {
                         quads[3] += 1;
                     }
                 }
                 quads
            });

        res.iter().product()
    }

    fn advance_robots(self:&Game, time: usize) -> HashSet<Point> {
        self.robots.iter()
            .map(|robot| robot.travel(time))//new coordinates of each robot on a global map
            .map(|position| self.wrap(position))//coordinates wrapped into same canvas
            .collect::<HashSet<Point>>()
    }
}


pub fn part_one(input: &str) -> Option<u32> {
    let game = Game {
        width: 101,
        height: 103,
        robots: input.lines().into_iter().map(|l| l.parse::<Robot>().unwrap()).collect_vec(),
    };

    Some(game.solve_linear(100) as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let game = Game {
        width: 101,
        height: 103,
        robots: input.lines().into_iter().map(|l| l.parse::<Robot>().unwrap()).collect_vec(),
    };

    let robots_count = game.robots.len();

    //lets be super dumb, and check if all points are un unique positions
    let mut time = 1;
    while time < 10000 {
        let advanced = game.advance_robots(time);
        if advanced.len() == robots_count {
            break;
        }
        time = time + 1;
    }

    Some(time as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
