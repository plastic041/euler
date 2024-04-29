mod helper;

use helper::math::Dividable;

pub fn solve(max_divisors: u64) -> u64 {
    let mut num: u64 = 0;
    let mut step = 1;

    loop {
        num += step;
        step += 1;

        if num.divisors().len() >= max_divisors as usize {
            return num;
        }
    }
}
