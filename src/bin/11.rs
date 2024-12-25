use std::{collections::HashMap, mem::swap};

use itertools::Itertools;

advent_of_code::solution!(11);

fn process_number(num:usize) -> Vec<usize> {
    match num {
        0 => vec![1],
        _ => {
            let number_as_str = num.to_string();
            if number_as_str.len() % 2 == 1 || num < 10 {
                vec![num * 2024]
            } else {
                vec![
                    number_as_str[0..number_as_str.len() / 2].parse().unwrap(),
                    number_as_str[number_as_str.len() / 2..number_as_str.len()].parse().unwrap(),
                ]
            }
        }
    }
}

fn solve_brute(data: &[usize], blink_number:usize) -> usize {
    let mut current:Vec<usize> = Vec::with_capacity(5000);
    current = data.to_vec().clone();
    let mut next:Vec<usize> = Vec::with_capacity(5000);

    for _ in  0..blink_number {
        next = current.iter().flat_map(|num| process_number(num.clone())).collect_vec();
        swap(&mut current, &mut next);
    }
    current.len()
}

fn dfs_count(num:usize, blink_remaining:usize, memo:&mut HashMap<(usize, usize), u64>) -> u64 {
    if memo.contains_key(&(num, blink_remaining)) {
        memo[&(num, blink_remaining)]
    } else if blink_remaining == 0 {
        return 1;
    } else {
        let result = match num {
            0 => dfs_count(1, blink_remaining - 1, memo),
            _ => {
                //let number_as_str = num.to_string();
                if !even_digits(num) || num < 10 {
                    dfs_count(num * 2024, blink_remaining - 1, memo)
                } else {
                    //let left = number_as_str[0..number_as_str.len() / 2].parse().unwrap();
                    //let right = number_as_str[number_as_str.len() / 2..number_as_str.len()].parse().unwrap();
                    let (left, right) = split_number(num);
                    dfs_count(left, blink_remaining - 1, memo) + dfs_count(right, blink_remaining - 1, memo)
                }
            }
        };
        memo.insert((num, blink_remaining), result);
        result
    }
}

fn solve_dfs(data: &[usize], blink_number:usize) -> usize {
    let mut memo:HashMap<(usize, usize), u64> = HashMap::new();
    data.into_iter()
        .fold(0, |acc, num| {
            let res = dfs_count(*num, blink_number, &mut memo) as usize;
            acc + res
        }
        )
}
fn solve_like_master_tought_me(data: Vec<usize>, iterations: usize) -> u64 {
    // Initialize frequency map
    let mut map: HashMap<usize, u64> = HashMap::new();
    for &val in &data {
        *map.entry(val).or_insert(0) += 1;
    }

    // Process iterations
    for _ in 0..iterations {
        let mut next_map = HashMap::new();

        for (&n, &count) in &map {
            if n == 0 {
                *next_map.entry(1).or_insert(0) += count;
            } else if even_digits(n) {
                let (left, right) = split_number(n);
                *next_map.entry(left).or_insert(0) += count;
                *next_map.entry(right).or_insert(0) += count;
            } else {
                *next_map.entry(n * 2024).or_insert(0) += count;
            }
        }

        map = next_map;
    }

    // Calculate the total count
    map.values().sum()
}

// Check if a number has even number of digits
fn even_digits(number: usize) -> bool {
    digits_len(number) % 2 == 0
}

// Calculate number of digits in a number
fn digits_len(mut number: usize) -> usize {
    if number == 0 {
        return 1;
    }
    let mut len = 0;
    while number > 0 {
        len += 1;
        number /= 10;
    }
    len
}

// Split the number into two halves
fn split_number(number: usize) -> (usize, usize) {
    let len = digits_len(number);
    let half_len = len / 2;
    let divisor = 10_i32.pow(half_len as u32) as usize;

    (number / divisor, number % divisor)
}

pub fn part_one(input: &str) -> Option<usize> {
    let data = input.split_whitespace().map(|number| number.parse::<usize>().unwrap()).collect_vec();
    //Some(solve_brute(data.as_ref(), 25))
    Some(solve_dfs(data.as_ref(), 75))
}

pub fn part_two(input: &str) -> Option<usize> {
    let data = input.split_whitespace().map(|number| number.parse::<usize>().unwrap()).collect_vec();
    //Some(solve_dfs(data.as_ref(), 25))
    //Some(solve_dfs(data.as_ref(), 75))
    Some(solve_like_master_tought_me(data, 75) as usize)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_process_number() {
        assert_eq!(process_number(0), vec![1]);
        assert_eq!(process_number(1), vec![2024]);
        assert_eq!(process_number(2), vec![2 * 2024]);
        assert_eq!(process_number(10), vec![1, 0]);
        assert_eq!(process_number(99), vec![9, 9]);
        assert_eq!(process_number(999), vec![2021976]);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(198075));
    }
}
