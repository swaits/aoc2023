use lazy_static::lazy_static;
use regex::Regex;

use crate::utils::digit_value;
use crate::Day;

inventory::submit! {
    Day { number: 1, run }
}

fn first_and_last_digit(s: &str) -> usize {
    let first = s
        .chars()
        .find(|c| c.is_ascii_digit())
        .unwrap()
        .to_digit(10)
        .unwrap() as usize;
    let last = s
        .chars()
        .rfind(|c| c.is_ascii_digit())
        .unwrap()
        .to_digit(10)
        .unwrap() as usize;

    first * 10 + last
}

fn day01_part1(input: &str) -> usize {
    input.lines().map(first_and_last_digit).sum()
}

lazy_static! {
    static ref REGEX: Regex =
        Regex::new(r"(?P<digit>zero|one|two|three|four|five|six|seven|eight|nine|\d)").unwrap();
}

fn first_and_last_digit_regex(s: &str) -> usize {
    let mut first_start = usize::MAX;
    let mut last_end = 0;
    let mut first_digit = String::new();
    let mut last_digit = String::new();

    for (index, _) in s.char_indices() {
        if let Some(mat) = REGEX.find(&s[index..]) {
            let match_start = index + mat.start();
            let match_end = index + mat.end();

            if match_start < first_start {
                first_start = match_start;
                first_digit = mat.as_str().to_string();
            }
            if match_end > last_end {
                last_end = match_end;
                last_digit = mat.as_str().to_string();
            }
        }
    }

    let first_digit_val = digit_value(&first_digit);
    let last_digit_val = digit_value(&last_digit);

    first_digit_val * 10 + last_digit_val
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
