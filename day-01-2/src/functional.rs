use std::str::FromStr;

/// Take a string slice and check if it starts by a digit in numeric
/// Return the found digit
fn read_value_in_digit(value: &str) -> Option<u32> {
    value.chars().nth(0).and_then(|c| c.to_digit(10))
}

/// Take a string slice and check if it starts by a digit written in letters
/// Return the found digit
fn read_value_in_letters(value: &str, digits_letter: &[String]) -> Option<u32> {
    digits_letter
        .iter()
        .enumerate()
        .find(|(_, repr)| value.starts_with(*repr))
        .map(|(index, _)| index as u32)
}

/// Take a string slice and check if it starts by a digit
/// Return the found digit
fn read_value(value: &str, digits_letter: &[String]) -> Option<u32> {
    read_value_in_digit(value).or_else(|| read_value_in_letters(value, digits_letter))
}

/// Take a string slice and find all the digits that can be found into it
/// Return an iterable that will iterate over each starting position in the string and check whether or not it corresponds to a digit
fn to_parsed_iter<'a>(
    value: &'a str,
    digits_letter: &'a [String],
) -> impl Iterator<Item = Option<u32>> + 'a {
    (0..value.len())
        .map(|index| &value[index..])
        .map(|sub_str| read_value(sub_str, digits_letter))
}

/// Find the first digit in a iterable on options
fn find_digit(mut parsed_digits: impl Iterator<Item = Option<u32>>) -> Option<u32> {
    parsed_digits.find(|parsed| parsed.is_some()).flatten()
}

fn extract_value(value: &str) -> u32 {
    let raw_digits_letter = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let digits_letter: [String; 10] =
        raw_digits_letter.map(|digit_str| String::from_str(digit_str).unwrap());
    let left_digit = find_digit(to_parsed_iter(value, &digits_letter));

    let rev_value: String = value.chars().rev().collect();
    let rev_digits_letter: [String; 10] =
        digits_letter.map(|digit_str| digit_str.chars().rev().collect());
    let right_digit = find_digit(to_parsed_iter(&rev_value, &rev_digits_letter));

    return left_digit.unwrap() * 10 + right_digit.unwrap();
}

pub fn run(corpus: &str) -> u32 {
    corpus.split('\n').map(extract_value).sum()
}
