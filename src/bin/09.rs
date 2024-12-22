use std::cmp;

use itertools::Itertools;

advent_of_code::solution!(9);

/// [Triangular numbers](https://en.wikipedia.org/wiki/Triangular_number) offset by two.
/// When evaluating n+1 elements  we can use triangular numbers, and hand code them as memoization:
const MEMO: [usize; 10] = [0, 0, 1, 3, 6, 10, 15, 21, 28, 36];

fn eval_sum(checksum: usize, block_id: usize, index: usize, size: usize) -> (usize, usize) {
    let id = index / 2;
    let extra = block_id * size + MEMO[size];
    (checksum + id * extra, block_id + size)
}

pub fn part_one(input: &str) -> Option<usize> {
    let input = input.trim().chars()
        .map(|ch| ch.to_digit(10).unwrap())
        .collect_vec();
    let mut left = 0;
    let mut right = input.len() - 2 + input.len() % 2;
    let mut to_move = input[right];
    let mut block_id = 0;

    let mut checksum:usize = 0;

    while left < right {
        //We are treating data as double linked list, where each node contains 2 elements:
        //first is current busy nodes, and we need to eval their checksum:
        (checksum, block_id) = eval_sum(checksum, block_id, left, input[left] as usize);
        //second part is free nodes, and we need to provess them by filling them with values from
        //tail of the linked list, by taking their busy nodes, and neglect their free nodes
        let mut free = input[left + 1];
        left += 2;

        while free > 0 {
            //find first non empty node from tail
            if to_move == 0 {
                if left == right {
                    break;
                }
                right -= 2;
                to_move = input[right];
            }

            //we can only move part, and need to evaluate checksum based on it
            let capacity = cmp::min(to_move, free);
            (checksum, block_id) = eval_sum(checksum, block_id, right, capacity as usize);
            free -= capacity;
            to_move -= capacity;
        }
    }
    //Process last node
    (checksum, _) = eval_sum(checksum, block_id, right, to_move as usize);
    Some(checksum)
}

pub fn part_two(input: &str) -> Option<i64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }
    #[test]
    fn check_len() {
        let input = "2333133121414131402";
        assert_eq!(9, input.len() / 2);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
