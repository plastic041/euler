use std::collections::HashMap;

mod helper;

pub fn solve(below: u32) -> u32 {
    let mut map = HashMap::new();
    map.insert(1, 1);

    for i in 2..=below {
        let mut n = i;
        let mut count = 0;

        loop {
            if let Some(&v) = map.get(&n) {
                count += v;
                break;
            }

            if n % 2 == 0 {
                n /= 2;
            } else {
                n = 3 * n + 1;
            }

            count += 1;
        }

        map.insert(i, count);
    }

    map.iter().max_by_key(|&(_, &v)| v).unwrap().0.to_owned()
}
