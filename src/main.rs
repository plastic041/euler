mod solve;

use solve::solve;

fn main() {
    let answer = solve(100);

    let diff = answer.1 - answer.0;

    println!("Answer: {:?}", diff);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let (sum_of_squares, square_of_sum) = solve(10);

        assert_eq!(sum_of_squares, 385);
        assert_eq!(square_of_sum, 3025);
    }
}
