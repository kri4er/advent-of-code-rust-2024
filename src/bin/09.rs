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

    let disk = input.trim().chars()
        .map(|ch| ch.to_digit(10).unwrap() as usize)
        .collect::<Vec<usize>>();

    let mut block = 0;
    let mut checksum = 0;
    let mut free: Vec<_> = (0..10).map(|_| Vec::with_capacity(1_100)).collect();

    // Build a min-heap (leftmost free block first) where the size of each block is
    // implicit in the index of the array.
    for (index, &size) in disk.iter().enumerate() {
        if index % 2 == 1 && size > 0 {
            free[size].push(block);
        }

        block += size;
    }

    // Add sentinel value and reverse vecs so that smallest blocks are last.
    for i in 0..10 {
        free[i].push(block);
        free[i].reverse();
    }

    for (index, &size) in disk.iter().enumerate().rev() {
        block -= size;

        // Count any previous free blocks to decrement block offset correctly.
        if index % 2 == 1 {
            continue;
        }

        // Find the leftmost free block that can fit the file (if any).
        let mut next_block = block;
        let mut next_index = usize::MAX;

        for i in size..free.len() {
            let top = free[i].len() - 1;
            let first = free[i][top];

            if first < next_block {
                next_block = first;
                next_index = i;
            }
        }

        // We can make smaller free block from bigger blocks but not the other way around.
        // As an optimization if all blocks of the biggest size are after our position then
        // we can ignore them.
        if !free.is_empty() {
            let biggest = free.len() - 1;
            let top = free[biggest].len() - 1;

            if free[biggest][top] > block {
                free.pop();
            }
        }

        // Update the checksum with the file's location (possibly unchanged).
        let id = index / 2;
        let extra = next_block * size + MEMO[size];
        checksum += id * extra;

        // If we used a free block, remove then add back any leftover space.
        if next_index != usize::MAX {
            free[next_index].pop();

            // Insert the new smaller block into the correct location.
            // Most frequently this is directly at the end of the vector so even though this
            // is technically `O(n)`, in practice it's faster than a real heap.
            let to = next_index - size;
            if to > 0 {
                let mut i = free[to].len();
                let value = next_block + size;

                while free[to][i - 1] < value {
                    i -= 1;
                }

                free[to].insert(i, value);
            }
        }
    }

    Some(checksum as i64)
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
        assert_eq!(result, Some(2858));
    }
}
