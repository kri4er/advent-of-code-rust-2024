use itertools::Itertools;

advent_of_code::solution!(7);

fn evaluate_expression(numbers: &[i64], operators: &[i64]) -> Option<i64> {
    if numbers.len() != operators.len() + 1 {
        return None;
    }

    let mut result = numbers[0];
    for (i, &op) in operators.iter().enumerate() {
        match op {
            0 => result += numbers[i + 1],
            1 => result *= numbers[i + 1],
            2 => result = (result.to_string() + numbers[i + 1].to_string().as_str()).parse::<i64>().unwrap(),
            _ => return None,
        }
    }

    Some(result)
}


pub fn part_one(input: &str) -> Option<i64> {
    let result:i64 = input.lines()
        .map(|l| {
            let(y,  operands) = l.split_once(':').unwrap();

            (y.parse::<i64>().unwrap(), operands
             .split_whitespace()
             .map(|o| o.trim())
             .map(|o| o.parse::<i64>().unwrap())
             .collect_vec())
        })
        .filter(|(y, operands)| {
            let operator_num:usize = operands.len() - 1;
            let possible_options = (0..operator_num).map(|_| 0..2).multi_cartesian_product().collect_vec();
            possible_options.iter().any(|option| {
                y == &evaluate_expression(operands, option).unwrap()
            })
        })
        .map(|(y, _operands)| y)
        .sum();

    Some(result as i64)
}

pub fn part_two(input: &str) -> Option<i64> {
    let result:i64 = input.lines()
        .map(|l| {
            let(y,  operands) = l.split_once(':').unwrap();

            (y.parse::<i64>().unwrap(), operands
             .split_whitespace()
             .map(|o| o.trim())
             .map(|o| o.parse::<i64>().unwrap())
             .collect_vec())
        })
        .filter(|(y, operands)| {
            let operator_num:usize = operands.len() - 1;
            let possible_options = (0..operator_num).map(|_| 0..3).multi_cartesian_product().collect_vec();
            possible_options.iter().any(|option| {
                y == &evaluate_expression(operands, option).unwrap()
            })
        })
        .map(|(y, _operands)| y)
        .sum();

    Some(result as i64)

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_permu() {
        let k = 2;
        let possible_options = (0..1).map(|_| 0..2).multi_cartesian_product().collect_vec();

        println!("LOMGE: operands: {:?}, options: {:?}", &k, &possible_options);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
