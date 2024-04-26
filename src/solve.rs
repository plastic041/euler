mod helper;

pub fn solve() -> Option<(u32, u32, u32)> {
    let max: u32 = 1_000;

    for a in 1..=max - 2 {
        for b in a..=max - 1 {
            let c = max - a - b;
            if a * a + b * b == c * c {
                // return (a, b, c);
                return Some((a, b, c));
            }
        }
    }

    None
}
