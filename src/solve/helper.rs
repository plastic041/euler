pub fn is_palindrome(number: u32) -> bool {
    let number_string = number.to_string();
    let reversed_number_string: String = number_string.chars().rev().collect();

    number_string == reversed_number_string
}
