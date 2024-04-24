mod helper;

pub fn solve(below: u32) -> u32 {
    (0..below).filter(|&x| x % 3 == 0 || x % 5 == 0).sum()
}
