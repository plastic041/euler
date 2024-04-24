mod helper;

use helper::lcm;

pub fn solve(max: u32) -> u32 {
    (1..=max).fold(1, |acc, x| lcm(acc, x))
}
