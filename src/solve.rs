mod helper;

use num_bigint::BigUint;

pub fn solve(below: u32) -> u32 {
    let num = BigUint::from(2u32).pow(below);

    num.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .sum()
}
