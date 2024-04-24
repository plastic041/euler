fn get_sieve_of_eratosthenes(limit: u64) -> Vec<u64> {
    let mut sieve = vec![true; limit as usize + 1];
    sieve[0] = false;
    sieve[1] = false;

    for i in 2..=(limit as f64).sqrt() as usize {
        if sieve[i] {
            for j in (i * i..=limit as usize).step_by(i) {
                sieve[j] = false;
            }
        }
    }

    sieve
        .iter()
        .enumerate()
        .filter(|(_, &is_prime)| is_prime)
        .map(|(i, _)| i as u64)
        .collect()
}

pub fn find_prime_factors(number: u64) -> Vec<u64> {
    let sqrt = (number as f64).sqrt() as u64;
    let primes = get_sieve_of_eratosthenes(sqrt);

    let factors = primes
        .into_iter()
        .filter(|&prime| number % prime == 0)
        .collect::<Vec<_>>();

    factors
}
