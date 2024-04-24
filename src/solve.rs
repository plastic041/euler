mod helper;

use helper::is_palindrome;

pub fn solve(min: u32, max: u32) -> Option<u32> {
    let mut largest_palindrome = 0;

    for i in min..=max {
        for j in min..=max {
            let product = i * j;

            if is_palindrome(product) && product > largest_palindrome {
                largest_palindrome = product;
            }
        }
    }

    if largest_palindrome == 0 {
        None
    } else {
        Some(largest_palindrome)
    }
}
