fn find_digit(mut chars: impl Iterator<Item = char>) -> Option<u32> {
    chars
        .find(|c| c.is_ascii_digit())
        .and_then(|c| c.to_digit(10))
}

fn extract_value(entry: &str) -> u32 {
    let first = find_digit(entry.chars().into_iter());
    let last = find_digit(entry.chars().rev().into_iter());
    return first.unwrap() * 10 + last.unwrap();
}

pub fn run(corpus: &str) -> u32 {
    corpus.split('\n').map(extract_value).sum()
}
