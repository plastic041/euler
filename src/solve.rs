mod helper;

pub fn solve(to: u32) -> (u32, u32) {
    let numbers = (1..=to).collect::<Vec<u32>>();

    let sum_of_squares: u32 = numbers.iter().map(|x| x.pow(2)).sum();
    let square_of_sum: u32 = numbers.iter().sum::<u32>().pow(2);

    (sum_of_squares, square_of_sum)
}
