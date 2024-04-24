mod solve;

use solve::solve;

fn main() {
    let answer = solve(100, 999).expect("No answer found");

    println!("Answer: {}", answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(solve(10, 99), Some(9009));
    }
}
