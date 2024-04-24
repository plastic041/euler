mod solve;

use solve::solve;

fn main() {
    let answer = solve(20);

    println!("Answer: {}", answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(solve(10), 2520);
    }
}
