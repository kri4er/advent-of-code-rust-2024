use std::{cmp::Ordering, collections::HashSet};

use itertools::Itertools;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let data = input.split_once("\n\n").unwrap();
    let rules:HashSet<(&str, &str)> = data.0.lines()
    //let rules:Vec<(&str, &str)> = data.0.lines()
        .map(|l| {
            let (f, s) = l.split_once('|').unwrap();
            (f, s)
        }
        )
        .collect();

    let updates = data.1.lines()
        .map(|l| l.split(',').collect_vec())
        .collect_vec();

    //println!("LOGME: updates: {:?}", &updates);
    let result:i32 = updates.iter()
        .filter(|update| {
            update.iter()
                .tuple_windows()
                .try_fold(1, |_is_ok, (l, r)|{
                    //println!("LOGME: tuple: {:?} and {:?}", l, r);
                    if rules.contains(&(r, l)) {
                        Err(())
                    } else {
                        Ok(1)
                    }
                }).is_ok()
        })
        .map(|update| update.get(update.len()/2)
            .unwrap()
            .parse::<i32>()
            .unwrap()
        )
        .sum();

    //println!("LOGME: rules: {:?}", rules);
    Some(result as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let data = input.split_once("\n\n").unwrap();
    let rules:HashSet<(&str, &str)> = data.0.lines()
        .map(|l| {
            let (f, s) = l.split_once('|').unwrap();
            (f, s)
        }
        )
        .collect();

    let mut updates:Vec<Vec<_>> = data.1.lines()
        .map(|l| l.split(',').collect_vec())
        .collect_vec();

    //println!("LOGME: updates: {:?}", &updates);
    let result:i32 = updates.into_iter()
        .filter(|update| {
            update.iter()
                .tuple_windows()
                .try_fold(1, |_is_ok, (l, r)|{
                    //println!("LOGME: tuple: {:?} and {:?}", l, r);
                    if rules.contains(&(r, l)) {
                        Err(())
                    } else {
                        Ok(1)
                    }
                }).is_err()
        })
        .map(|update| {
            let mut sorted = update.clone();
            sorted.sort_by(|&l, &r| {
                if rules.contains(&(l, r)) {
                    return Ordering::Less;
                } else if rules.contains(&(r, l)) {
                    return Ordering::Greater;
                }
                return Ordering::Equal;
            });
            sorted
        })
        .map(|update| update.get(update.len()/2)
            .unwrap()
            .parse::<i32>()
            .unwrap()
        )
        .sum();

    //println!("LOGME: rules: {:?}", rules);
    Some(result as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
