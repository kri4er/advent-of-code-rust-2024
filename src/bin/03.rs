use regex::Regex;

advent_of_code::solution!(3);
pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();

    let matches: Vec<(i32, i32)> = re
        .captures_iter(input)
        .map(|cap| {
            let x: i32 = cap[1].parse().unwrap();
            let y: i32 = cap[2].parse().unwrap();
            (x, y)
        })
        .collect();

    let result = matches.into_iter()
        .fold(0, |acc, (x, y)|
            acc + (x * y)
            );
    Some(result as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r"(mul\(([0-9]+),([0-9]+)\)|do\(\)|don't\(\))").unwrap();
    let mut enabled = true;

    let matches: Vec<(i32, i32)> = re
        .captures_iter(input)
        .filter(|cap|{
            if cap.get(0).unwrap().as_str().as_bytes() == b"do()" {
                enabled = true;
                return false;
            } else if cap.get(0).unwrap().as_str().as_bytes() == b"don't()" {
                enabled = false;
            }
            enabled
        })
        .map(|cap| {
            let x: i32 = cap[2].parse().unwrap();
            let y: i32 = cap[3].parse().unwrap();
            (x, y)
        })
        .collect();

    let result = matches.into_iter()
        .fold(0, |acc, (x, y)|
            acc + (x * y)
            );
    Some(result as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(48));
    }
}
