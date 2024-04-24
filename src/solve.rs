mod helper;

use helper::{is_even, Fibonacci};

pub fn solve(below: u32) -> u64 {
    let mut fibonacci = Fibonacci::new();

    loop {
        let next = fibonacci.next();
        if next > below as u64 {
            break;
        }
        fibonacci.push(next);
    }

    let evens = fibonacci
        .numbers
        .into_iter()
        .filter(|&x| is_even(x))
        .collect::<Vec<_>>();

    evens.into_iter().sum()
}
