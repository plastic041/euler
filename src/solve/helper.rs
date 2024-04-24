pub struct Fibonacci {
    pub numbers: Vec<u64>,
}

impl Fibonacci {
    pub fn new() -> Self {
        let mut numbers = Vec::new();
        numbers.push(1);
        numbers.push(2);

        Fibonacci { numbers }
    }

    pub fn next(&mut self) -> u64 {
        let len = self.numbers.len();
        let next = self.numbers[len - 1] + self.numbers[len - 2];

        next
    }

    pub fn push(&mut self, n: u64) {
        self.numbers.push(n);
    }
}

pub fn is_even(n: u64) -> bool {
    let result = n % 2 == 0;

    result
}
