mod solve;

use solve::solve;

fn main() {
    let answer = solve(10_001);

    println!("Answer: {:?}", answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let answer = solve(6);

        assert_eq!(answer, 13);
    }
}
