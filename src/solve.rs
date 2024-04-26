mod helper;

fn make_sieve(below: usize) -> Vec<bool> {
    let mut sieve = vec![true; below];
    sieve[0] = false;
    sieve[1] = false;

    for i in 2..(below as f64).sqrt() as usize {
        if sieve[i] {
            for j in (i * i..below).step_by(i) {
                sieve[j] = false;
            }
        }
    }

    sieve
}

pub fn solve(below: usize) -> u64 {
    let sieve = make_sieve(below);
    let mut sum = 0;

    dbg!(&sieve);

    for i in 0..below {
        if sieve[i] {
            sum += i as u64;
        }
    }

    sum
}
