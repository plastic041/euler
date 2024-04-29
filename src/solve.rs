mod helper;

use num_bigint::BigUint;

pub fn solve(numbers_string: String) -> String {
    let numbers = numbers_string
        .lines()
        .map(|line| line.parse::<BigUint>().expect("Failed to parse number"))
        .collect::<Vec<BigUint>>();

    let sum = numbers.iter().sum::<BigUint>();

    let sum_string = sum.to_string();

    sum_string.chars().take(10).collect()
}
