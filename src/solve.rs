mod helper;

use helper::find_prime_factors;

pub fn solve(number: u64) -> u64 {
    find_prime_factors(number).pop().unwrap()
}
