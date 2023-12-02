use color_eyre::{eyre::eyre, Result};
use std::path::Path;

fn digit_words() -> [(&'static str, u32); 9] {
    [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]
}

fn try_from_word_to_digit(s: &str) -> Option<u32> {
    digit_words()
        .iter()
        .find(|(word, _)| s.starts_with(word))
        .map(|(_, digit)| *digit)
}

fn try_from_numeric_to_digit(s: &str) -> Option<u32> {
    s.chars().next().and_then(|c| c.to_digit(10))
}

fn try_from_word_or_numeric_to_digit(s: &str) -> Option<u32> {
    try_from_numeric_to_digit(s).or_else(|| try_from_word_to_digit(s))
}

fn compute_number_from_first_and_last_digit<F>(s: &str, is_digit_fn: F) -> Result<u32>
where
    F: Fn(&str) -> Option<u32>,
{
    let chunker = || (0..s.len()).map(|start_pos| &s[start_pos..]);

    let first = chunker().find_map(&is_digit_fn);
    let last = chunker().rev().find_map(&is_digit_fn);

    //  {
    //     let chunk = &s[i..s.len()];
    //     let maybe_digit = is_digit_fn(chunk);
    //     if maybe_digit.is_some() {
    //         if first.is_none() {
    //             first = maybe_digit
    //         }
    //         last = maybe_digit
    //     }
    // }
    let number = match (first, last) {
        (Some(first), Some(last)) => first * 10 + last,
        _ => return Err(eyre!("some digit not found")),
    };

    Ok(number)
}

fn compute_number_from_first_and_last_digit_numeric(s: &str) -> Result<u32> {
    compute_number_from_first_and_last_digit(s, try_from_numeric_to_digit)
}

fn compute_number_from_first_and_last_digit_word_numeric(s: &str) -> Result<u32> {
    compute_number_from_first_and_last_digit(s, try_from_word_or_numeric_to_digit)
}

fn sum_from_two_digit_numbers<F>(s: &str, number_finder: F) -> Result<u32>
where
    F: Fn(&str) -> Result<u32>,
{
    let mut sum = 0;
    for line in s.lines() {
        sum += number_finder(line)?;
    }
    Ok(sum)
}

fn sum_from_two_digit_numbers_numeric(s: &str) -> Result<u32> {
    sum_from_two_digit_numbers(s, compute_number_from_first_and_last_digit_numeric)
}

fn sum_from_two_digit_numbers_word_numeric(s: &str) -> Result<u32> {
    sum_from_two_digit_numbers(s, compute_number_from_first_and_last_digit_word_numeric)
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let s = std::fs::read_to_string(input)?;
    let res = sum_from_two_digit_numbers_numeric(&s)?;
    println!("part 1: {res}");
    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    let s = std::fs::read_to_string(input)?;
    let res = sum_from_two_digit_numbers_word_numeric(&s)?;
    println!("part 2: {res}");
    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("no solution found")]
    NoSolution,
    #[error(transparent)]
    EyreReport(#[from] color_eyre::eyre::Error),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> Result<(), Error> {
        let s = "1abc2";
        assert_eq!(sum_from_two_digit_numbers_numeric(s)?, 12);

        let s = "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";
        assert_eq!(sum_from_two_digit_numbers_numeric(s)?, 142);

        let s = "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen";
        assert_eq!(sum_from_two_digit_numbers_word_numeric(s)?, 281);

        Ok(())
    }
}
