advent_of_code::solution!(2);
use itertools::Itertools;


fn is_decreasing(path: &Vec<i64>) -> bool {
    for i in 0..path.len() - 1 {
        if (path[i] > path[i+1]) && (path[i] - path[i + 1]) < 4 {
            continue;
        } else {
            return false;
        }
    }
    true
}

fn is_increasing(path: &Vec<i64>) -> bool {
    for i in 0..path.len() - 1 {
        if (path[i] < path[i+1]) && (path[i + 1] - path[i]) < 4 {
            continue;
        } else {
            return false;
        }
    }
    true
}

fn is_safe(path: &Vec<i64>) -> bool {
    if path.len() < 2 { true } else {
        is_decreasing(&path) || is_increasing(&path)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let result:u32 = input.lines()
        .map(|line|
            line.split_whitespace()
            .map(|num| num.parse::<i64>().unwrap()).collect::<Vec<i64>>())
        .map(|parsed_slope| if is_safe(&parsed_slope) {1} else {0})
        .sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let result:usize = input.lines()
        .map(|line|
            line.split_whitespace()
            .map(|num| num.parse::<i64>().unwrap()).collect::<Vec<i64>>())
        .filter(|nums| {
            (0..nums.len()).any(|i| {
                nums[0..i]
                    .iter()
                    .chain(&nums[i + 1..])
                    .tuple_windows()
                    .try_fold(0, |ord, (a, b)| {
                        if ord >= 0 && (1..=3).contains(&(b - a)) {
                            Ok(1)
                        } else if ord <= 0 && (1..=3).contains(&(a - b)) {
                            Ok(-1)
                        } else {
                            Err(())
                        }
                    })
                .is_ok()
            })
        }
        ).count();
    Some(result as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
