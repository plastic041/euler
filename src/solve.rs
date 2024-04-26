mod helper;

use helper::math::is_prime;

pub fn solve(nth: usize) -> u32 {
    let mut count = 0;
    let mut i = 2;

    while count < nth {
        if is_prime(i) {
            count += 1;
        }

        i += 1;
    }

    i - 1
}
