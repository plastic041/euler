mod solve;

use solve::solve;

fn main() {
    let answer = solve(1_000_000);

    println!("Answer: {:?}", answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let answer = solve(13);

        assert_eq!(answer, 10);
    }
}
