mod data;

fn next_number_range(line: &str) -> Option<(usize, usize)> {
    match line.find(|c: char| c.is_ascii_digit()) {
        None => None,
        Some(digit_start) => match &line[digit_start..].find(|c: char| !c.is_ascii_digit()) {
            None => Some((digit_start, line.len())),
            Some(digit_end) => Some((digit_start, digit_start + *digit_end)),
        },
    }
}

fn extract_number_ranges(line: &str) -> Vec<(usize, usize)> {
    let mut current_pos: usize = 0;
    let mut ranges: Vec<(usize, usize)> = Vec::new();
    loop {
        let range = next_number_range(&line[current_pos..]);
        match range {
            None => return ranges,
            Some(range) => {
                ranges.push((current_pos + range.0, current_pos + range.1));
                current_pos += range.1;
            }
        }
    }
}

fn is_symbol(matrix: &Vec<Vec<char>>, line_index: usize, row_index: usize) -> bool {
    match matrix.get(line_index) {
        None => false,
        Some(line_chars) => match line_chars.get(row_index) {
            None => false,
            Some(c) => c != &'.' && !c.is_ascii_digit(),
        },
    }
}

fn run(corpus: &str) -> u32 {
    let matrix: Vec<Vec<char>> = corpus
        .split('\n')
        .map(|str| -> Vec<char> { str.chars().into_iter().collect() })
        .collect();

    let mut sum: u32 = 0;
    for (line_index, line) in corpus.split('\n').enumerate() {
        let ranges = extract_number_ranges(line);
        'range_loop: for range in ranges {
            if range.0 > 0 && is_symbol(&matrix, line_index, range.0 - 1) {
                sum += &line[range.0..range.1].parse().unwrap_or_default();
                continue 'range_loop;
            }
            if is_symbol(&matrix, line_index, range.1) {
                sum += &line[range.0..range.1].parse().unwrap_or_default();
                continue 'range_loop;
            }
            let range_min = if range.0 > 0 { range.0 - 1 } else { 0 };
            if line_index > 0 {
                for row_index in range_min..(range.1 + 1) {
                    if is_symbol(&matrix, line_index - 1, row_index) {
                        sum += &line[range.0..range.1].parse().unwrap_or_default();
                        continue 'range_loop;
                    }
                }
            }
            for row_index in range_min..(range.1 + 1) {
                if is_symbol(&matrix, line_index + 1, row_index) {
                    sum += &line[range.0..range.1].parse().unwrap_or_default();
                    continue 'range_loop;
                }
            }
        }
    }
    sum
}

fn main() {
    let input = data::get();

    let response = run(input);
    println!("Response #1 is: {response}");
    assert_eq!(response, 527369);
}
