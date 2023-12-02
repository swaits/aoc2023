use lazy_static::lazy_static;
use regex::Regex;

use crate::{utils::digit_value, Day};

// Register this day in our runner inventory
inventory::submit! {
    Day { number: 1, run }
}

fn first_and_last_digit(s: &str) -> usize {
    // Find the first ASCII digit in the string
    let first = s.chars().filter_map(|c| c.to_digit(10)).next().unwrap() as usize; // Safe to use unwrap as a digit is always present

    // Find the last ASCII digit in the string
    let last = s
        .chars()
        .rev() // Reverse the iteration for the last digit
        .filter_map(|c| c.to_digit(10))
        .next()
        .unwrap() as usize; // Safe to use unwrap here as well

    // Combine the first and last digits to form a new number
    first * 10 + last
}

lazy_static! {
    static ref REGEX: Regex =
        Regex::new(r"(?P<digit>zero|one|two|three|four|five|six|seven|eight|nine|\d)").unwrap();
}

fn first_and_last_digit_regex(s: &str) -> usize {
    // Find the first digit in the string by testing against each sice from [0..] to [n-1..]
    let first_digit = s
        .char_indices()
        .find_map(|(index, _)| REGEX.find(&s[index..]).map(|mat| mat.as_str().to_string()))
        .unwrap(); // Safe to use unwrap because a digit is always expected

    // Iterate from the end of the string, checking each slice for the last match
    let last_digit = s
        .char_indices()
        .rev()
        .find_map(|(index, _)| {
            // Check if the slice from this index to the end of the string matches the regex
            REGEX.find(&s[index..]).map(|mat| mat.as_str().to_string())
        })
        .unwrap(); // Safe to use unwrap as a digit is always expected

    // Convert the found digits to numbers and calculate the result
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
