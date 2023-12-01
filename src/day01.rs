use lazy_static::lazy_static;
use regex::Regex;

use crate::{utils::digit_value, Day};

// Register this day in our runner inventory
inventory::submit! {
    Day { number: 1, run }
}

fn first_and_last_digit(s: &str) -> usize {
    // Find the first character in the string that is an ASCII digit
    let first = s
        .chars()
        .find(|c| c.is_ascii_digit()) // Use find to get an Option containing the first digit character
        .unwrap() // Unwrap the Option, panicking if no digit is found
        .to_digit(10) // Convert the character to a digit in base 10
        .unwrap() as usize; // Convert the digit to a usize

    // Find the last character in the string that is an ASCII digit
    let last = s
        .chars()
        .rfind(|c| c.is_ascii_digit()) // Use rfind to get an Option containing the last digit character
        .unwrap() // Unwrap the Option, panicking if no digit is found
        .to_digit(10) // Convert the character to a digit in base 10
        .unwrap() as usize; // Convert the digit to a usize

    // Return the result
    first * 10 + last
}

lazy_static! {
    static ref REGEX: Regex =
        Regex::new(r"(?P<digit>zero|one|two|three|four|five|six|seven|eight|nine|\d)").unwrap();
}

fn first_and_last_digit_regex(s: &str) -> usize {
    // Initialize variables to keep track of the positions and values of the first and last digits.
    let mut first_start = usize::MAX;
    let mut last_end = 0;
    let mut first_digit = String::new();
    let mut last_digit = String::new();

    // Iterate over each character index in the string.
    for (index, _) in s.char_indices() {
        // Check if there is a digit at the current index or beyond.
        if let Some(mat) = REGEX.find(&s[index..]) {
            // Calculate the start and end positions of the matched digit.
            let match_start = index + mat.start();
            let match_end = index + mat.end();

            // If the current match starts before any previously found digit, it's the new first digit.
            if match_start < first_start {
                first_start = match_start;
                first_digit = mat.as_str().to_string();
            }
            // If the current match ends after any previously found digit, it's the new last digit.
            if match_end > last_end {
                last_end = match_end;
                last_digit = mat.as_str().to_string();
            }
        }
    }

    // Convert the first and last digit strings to their numeric values and return the result
    digit_value(&first_digit) * 10 + digit_value(&last_digit)
}

fn day01_part1(input: &str) -> usize {
    input.lines().map(first_and_last_digit).sum()
}

fn day01_part2(input: &str) -> usize {
    input.lines().map(first_and_last_digit_regex).sum()
}

const INPUT: &str = include_str!("../data/day01.dat");

fn run() {
    println!("{}", day01_part1(INPUT));
    println!("{}", day01_part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day01_part1() {
        let input = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
        assert_eq!(day01_part1(input), 142);
        assert_eq!(day01_part1(INPUT), 54597);
    }

    #[test]
    fn test_day01_part2() {
        let input="two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";
        assert_eq!(day01_part2(input), 281);
        assert_eq!(day01_part2(INPUT), 54504);
    }
}
