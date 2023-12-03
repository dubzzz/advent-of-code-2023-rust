fn extract_left_value(entry: &str) -> u32 {
    let digits_letter = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    for length in 1..=entry.len() {
        let sub_str_start: &str = &entry[..length];
        for (index, repr) in digits_letter.iter().enumerate() {
            if sub_str_start.ends_with(repr) {
                return index as u32;
            }
        }
        let last = sub_str_start.chars().last();
        if last.map(|c| c.is_ascii_digit()).unwrap_or_default() {
            return last.and_then(|c| c.to_digit(10)).unwrap();
        }
    }
    return 0;
}

fn extract_right_value(entry: &str) -> u32 {
    let digits_letter = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    for length in 1..=entry.len() {
        let start = entry.len() - length;
        let sub_str_end: &str = &entry[start..];
        for (index, repr) in digits_letter.iter().enumerate() {
            if sub_str_end.starts_with(repr) {
                return index as u32;
            }
        }
        let first = sub_str_end.chars().nth(0);
        if first.map(|c| c.is_ascii_digit()).unwrap_or_default() {
            return first.and_then(|c| c.to_digit(10)).unwrap();
        }
    }
    return 0;
}

fn extract_value(entry: &str) -> u32 {
    return extract_left_value(entry) * 10 + extract_right_value(entry);
}

pub fn run(corpus: &str) -> u32 {
    corpus.split('\n').map(extract_value).sum()
}
