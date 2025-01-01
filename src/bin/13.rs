use std::{cmp::min, num::ParseIntError, ops::{Add, Mul, Sub}, str::FromStr};

advent_of_code::solution!(13);

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

impl Mul<i64> for Point {
    type Output = Point;

    fn mul(self, other:i64) -> Point {
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

    pub fn cost(p:&Point) -> i64 {
        p.0*3 + p.1*1
    }

    pub fn solve_brute(&self, multi:i64) -> Option<Point> {
        let mut best_value = i64::max_value();
        let mut best_actions = Point(0, 0);
        let press_a = Point(1, 0);
        let press_b = Point(0, 1);
        let prize = self.prize + multi;

        let mut cur_point = Point(0, 0);
        while self.a * cur_point.0 < prize {
            while self.b * cur_point.1 < prize {
                let cur_point_value = (self.a * cur_point.0) + (self.b * cur_point.1);
                if  cur_point_value == prize {
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

    pub fn solve_equation(&self, multi:i64) -> Option<Point> {
        let prize = self.prize + multi;
        /*
         T.x = press_a * A.x + press_b * B.x
         T.y = press_a * A.y + press_b * B.y

         B.y * T.x = B.y * press_a * A.x + B.y * press_b * B.x
         B.x * T.y = B.x * press_a * A.y + B.x * press_b * B.y

         B.x * T.y - B.y * T.x = press_a * A.y * B.x - press_a * A.x * B.y
         B.x * T.y - B.y * T.x = press_a * (Ay * Bx - Ax * By)
         (B.x * T.y - B.y * T.x) / (Ay * Bx - Ax * By) = press_a

         */
        let press_a:f64 = (self.b.0 * prize.1 - self.b.1 * prize.0) as f64
            / (self.a.1 * self.b.0 - self.a.0 * self.b.1) as f64;
        //println!("press a: {}", press_a);
        //// Find pressB
        // T.x = pressA * A.x + pressB * B.x
        // T.x - pressA * A.x = pressB * B.x
        // (T.x - pressA * A.x) / B.x = pressB
        if press_a.fract() != 0f64 {
            //println!("-- press a fraction is : {}", press_a.fract());
            return None
        }
        let press_b = (prize.0 as f64 - press_a * self.a.0 as f64) / self.b.0 as f64;

        // println!("press b: {}", press_b);
        if press_b.fract() != 0f64 {
            // println!("-- press b fraction is : {}", press_b.fract());
            return None
        }

        Some(Point(press_a as i64, press_b as i64))
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
    let res:i64 = input
        .split("\n\n")
        .map(|game| game.parse::<Game>().unwrap())
        //.map(|game| game.solve_brute(0))
        .map(|game| game.solve_equation(0))
        .filter(|game| game.is_some())
        .map(|p| Game::cost(&p.unwrap()))
        .sum();

    Some(res as u32)
}

pub fn part_two(input: &str) -> Option<i64> {
    let res:i64 = input
        .split("\n\n")
        .map(|game| game.parse::<Game>().unwrap())
        .map(|game| game.solve_equation(10000000000000))
        .filter(|game| game.is_some())
        .map(|p| Game::cost(&p.unwrap()))
        .sum();

    Some(res as i64)
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
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, None);
    }
}
