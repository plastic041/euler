mod solve;

use solve::solve;

fn main() {
    let answer = solve(600851475143);

    println!("Answer: {}", answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(solve(13195), 29);
    }
}
