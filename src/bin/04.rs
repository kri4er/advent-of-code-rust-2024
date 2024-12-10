use std::collections::HashMap;

advent_of_code::solution!(4);

#[derive(Debug, PartialEq)]
pub struct Point(i32, i32);

static DIRS: &'static [[Point; 4]] = &[
    [Point(0, 0), Point(1, -1), Point(2, -2), Point(3, -3),],
    [Point(0, 0), Point(1, 0), Point(2, 0), Point(3, 0),],
    [Point(0, 0), Point(1, 1), Point(2, 2), Point(3, 3),],
    [Point(0, 0), Point(0, 1), Point(0, 2), Point(0, 3),],
];

fn vec_to_hashmap(data: &Vec<&str>) -> HashMap<(i32, i32), char> {
    data.iter()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(move |(col, ch)|
                    ((row as i32, col as i32), ch)
                )
        })
        .collect()
}


pub fn part_one(input: &str) -> Option<u32> {
    let m = input.lines()
        .collect::<Vec<_>>();

    let m:HashMap<(i32, i32), char> = vec_to_hashmap(&m);

    let result:usize = m.keys()
        .map(|(x, y)| {
            DIRS.iter()
                .filter(|p| {
                    let word = &format!("{}{}{}{}",
                        m.get(&(x + p[0].0, y + p[0].1)).unwrap_or(&'.'),
                        m.get(&(x + p[1].0, y + p[1].1)).unwrap_or(&'.'),
                        m.get(&(x + p[2].0, y + p[2].1)).unwrap_or(&'.'),
                        m.get(&(x + p[3].0, y + p[3].1)).unwrap_or(&'.'),
                    );
                    word == "XMAS" || word == "SAMX"
            }).count()
        }).sum();

    Some(result as u32)
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
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
