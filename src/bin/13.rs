use std::{cmp::min, num::ParseIntError, ops::{Add, Mul, Sub}, str::FromStr};

use itertools::Itertools;
use nom::error::ParseError;

advent_of_code::solution!(13);

#[derive(Debug, PartialEq, Hash, Eq, Clone, Copy, PartialOrd, Ord)]
pub struct Point(i32, i32);


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

impl Mul for Point {
    type Output = Point;

    fn mul(self, other:Point) -> Point {
        Point(self.0 * other.0, self.1 * other.1)
    }

}

impl Mul<i32> for Point {
    type Output = Point;

    fn mul(self, other:i32) -> Point {
        Point(self.0 * other, self.1 * other)
    }

}


#[derive(Debug, PartialEq)]
pub struct Game {
    a: Point,
    b: Point,
    prize: Point,
}

impl Game {
    fn parse_button(btn:&str) -> Result<Point, ParseIntError> {
        let (_, coords) = btn.split_once(':').unwrap();
        let (x, y) = coords.split_once(',').unwrap();

        Ok(
            Point(
                x.trim().strip_prefix("X+").unwrap().parse()?,
                y.trim().strip_prefix("Y+").unwrap().parse()?,
            )
        )
    }
    fn parse_prize(btn:&str) -> Result<Point, ParseIntError> {
        let (_, coords) = btn.split_once(':').unwrap();
        let (x, y) = coords.split_once(',').unwrap();

        Ok(
            Point(
                x.trim().strip_prefix("X=").unwrap().parse()?,
                y.trim().strip_prefix("Y=").unwrap().parse()?,
            )
        )
    }

    pub fn cost(p:&Point) -> i32 {
        p.0*3 + p.1*1
    }

    pub fn solve_brute(&self) -> Option<Point> {
        let mut best_value = i32::max_value();
        let mut best_actions = Point(0, 0);
        let press_a = Point(1, 0);
        let press_b = Point(0, 1);

        let mut cur_point = Point(0, 0);
        while self.a * cur_point.0 < self.prize {
            while self.b * cur_point.1 < self.prize {
                let cur_point_value = (self.a * cur_point.0) + (self.b * cur_point.1);
                if  cur_point_value == self.prize {
                    if best_value > Game::cost(&cur_point_value) {
                        best_value = Game::cost(&cur_point_value);
                        best_actions = cur_point.clone();
                    }
                }
                cur_point = cur_point + press_b;
            }
            //reset button b
            cur_point = Point(cur_point.0, 0);
            cur_point = cur_point + press_a;
        }
        Some(best_actions)
    }

}

impl FromStr for Game {
    type Err = std::num::ParseIntError;


    fn from_str(data: &str) -> Result<Self, Self::Err> {
        let mut it = data.splitn(3, '\n');

        Ok(Game {
            a: Game::parse_button(it.next().unwrap())?,
            b: Game::parse_button(it.next().unwrap())?,
            prize: Game::parse_prize(it.next().unwrap())?,
        })
    }

}

pub fn part_one(input: &str) -> Option<u32> {
    let res:i32 = input
        .split("\n\n")
        .map(|game| game.parse::<Game>().unwrap())
        .map(|game| game.solve_brute().unwrap())
        .map(|p| Game::cost(&p))
        .sum();

    Some(res as u32)
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
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
