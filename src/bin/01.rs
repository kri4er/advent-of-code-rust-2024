use std::collections::{hash_map, HashMap};


advent_of_code::solution!(1);


pub fn part_one(input: &str) -> Option<u32> {
    let star_list:Vec<&str> = input.lines().collect();
    let star_list:Vec<u32> = star_list.into_iter()
        .flat_map(|l|
            l.split_whitespace()
            .map(|num| num.trim().parse::<u32>().unwrap())
            .collect::<Vec<u32>>()
            ).collect();

    let mut left:Vec<u32> = vec![];
    let mut right:Vec<u32> = vec![];

    for i in star_list.chunks(2) {
        left.push(i[0]);
        right.push(i[1]);
    }

    left.sort();
    right.sort();
    let result = left.into_iter()
        .zip(right.into_iter())
        .map(|(l, r)|
            (l as i64 - r as i64).abs()
            ).sum::<i64>();


    Some(result as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let star_list:Vec<&str> = input.lines().collect();
    let star_list:Vec<u32> = star_list.into_iter()
        .flat_map(|l|
            l.split_whitespace()
            .map(|num| num.trim().parse::<u32>().unwrap())
            .collect::<Vec<u32>>()
            ).collect();

    let mut m:HashMap<u32, u32> = HashMap::new();
    let mut right:Vec<u32> = vec![];

    for i in star_list.chunks(2) {
        m.insert(i[1], if m.contains_key(&i[1]) { m[&i[1]] + 1 } else { 1 });
        right.push(i[0]);
    }

    let result = right.iter()
        .map(|r|
            if m.contains_key(&r) { (m[&r] * r) as i64} else { 0 }
            ).sum::<i64>();


    Some(result as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
