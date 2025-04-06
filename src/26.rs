// Problem: Write a function to check if a given string is a palindrome or not.
use std::cmp;

/// Determines if a given string is a palindrome.
///
/// # Arguments
///
/// * `input_str` - A string representing the input to be checked.
///
/// # Returns
///
/// * `bool` - True if the input string is a palindrome, false otherwise.
fn is_palindrome(input_str: &str) -> bool {
    let len = input_str.len();
    for i in 0..len / 2 {
        if input_str.chars().nth(i).unwrap() != input_str.chars().nth(len - 1 - i).unwrap() {
            return false;
        }
    }
    true
}

/// Main function to check the correctness of the is_palindrome function.
fn main() {
    let test_cases = vec!["racecar", "hello", "world", "madam"];
    for &test_case in &test_cases {
        println!("Is '{}' a palindrome? {}", test_case, is_palindrome(test_case));
    }
}
