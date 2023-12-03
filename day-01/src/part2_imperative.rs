use std::str::FromStr;

fn find_digit(value: &str, digits_letter: &[String]) -> Option<u32> {
    for length in 1..=value.len() {
        let sub_str_start: &str = &value[..length];
        for (index, repr) in digits_letter.iter().enumerate() {
            if sub_str_start.ends_with(repr) {
                return Some(index as u32);
            }
        }
        let last = sub_str_start.chars().last();
        let last_as_number = last.and_then(|c| c.to_digit(10));
        if last_as_number.is_some() {
            return last_as_number;
        }
    }
    return None;
}

fn extract_value(value: &str) -> u32 {
    let raw_digits_letter = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let digits_letter: [String; 10] =
        raw_digits_letter.map(|digit_str| String::from_str(digit_str).unwrap());
    let left_digit = find_digit(value, &digits_letter);

    let rev_value: String = value.chars().rev().collect();
    let rev_digits_letter: [String; 10] =
        digits_letter.map(|digit_str| digit_str.chars().rev().collect());
    let right_digit = find_digit(&rev_value, &rev_digits_letter);

    return left_digit.unwrap() * 10 + right_digit.unwrap();
}

pub fn run(corpus: &str) -> u32 {
    corpus.split('\n').map(extract_value).sum()
}
