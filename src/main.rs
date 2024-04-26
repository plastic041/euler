mod solve;

use solve::solve;

fn main() {
    let answer = solve();

    println!("Answer: {:?}", answer);
    println!(
        "Product: {}",
        answer.map(|(a, b, c)| a * b * c).unwrap_or(0)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        // let answer = solve(4);

        // assert_eq!(answer, 5832);
    }
}
