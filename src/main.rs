mod solve;

use solve::solve;

fn main() {
    let answer = solve(500);

    println!("Answer: {:?}", answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let answer = solve(5);

        assert_eq!(answer, 28);
    }
}
