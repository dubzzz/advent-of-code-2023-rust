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

pub fn run(corpus: &str) -> u32 {
    let matrix: Vec<Vec<char>> = corpus
        .split('\n')
        .map(|str| -> Vec<char> { str.chars().into_iter().collect() })
        .collect();
    let ranges_per_line: Vec<Vec<(usize, usize)>> =
        corpus.split('\n').map(extract_number_ranges).collect();

    let mut sum: u32 = 0;
    for (line_index, line) in matrix.iter().enumerate() {
        for (row_index, c) in line.iter().enumerate() {
            if c != &'*' {
                continue;
            }

            let mut nums: Vec<u32> = Vec::new();

            // Line Above
            if line_index > 0 {
                let empty: Vec<(usize, usize)> = Vec::new();
                let ranges_above = ranges_per_line.get(line_index - 1).unwrap_or(&empty);
                for range in ranges_above {
                    if (row_index > 0 && range.0 <= row_index - 1 && row_index - 1 < range.1)
                        || (range.0 <= row_index && row_index < range.1)
                        || (range.0 <= row_index + 1 && row_index + 1 < range.1)
                    {
                        let line = matrix.get(line_index - 1).unwrap();
                        nums.push(
                            String::from_iter(&line[range.0..range.1])
                                .parse::<u32>()
                                .unwrap_or_default(),
                        )
                    }
                }
            }

            // Current Line
            let empty: Vec<(usize, usize)> = Vec::new();
            let ranges_current = ranges_per_line.get(line_index).unwrap_or(&empty);
            for range in ranges_current {
                if (row_index > 0 && range.0 <= row_index - 1 && row_index - 1 < range.1)
                    || (range.0 <= row_index && row_index < range.1)
                    || (range.0 <= row_index + 1 && row_index + 1 < range.1)
                {
                    nums.push(
                        String::from_iter(&line[range.0..range.1])
                            .parse::<u32>()
                            .unwrap_or_default(),
                    )
                }
            }

            // Line Below
            let empty: Vec<(usize, usize)> = Vec::new();
            let ranges_below = ranges_per_line.get(line_index + 1).unwrap_or(&empty);
            for range in ranges_below {
                if (row_index > 0 && range.0 <= row_index - 1 && row_index - 1 < range.1)
                    || (range.0 <= row_index && row_index < range.1)
                    || (range.0 <= row_index + 1 && row_index + 1 < range.1)
                {
                    let line = matrix.get(line_index + 1).unwrap();
                    nums.push(
                        String::from_iter(&line[range.0..range.1])
                            .parse::<u32>()
                            .unwrap_or_default(),
                    )
                }
            }

            if nums.len() == 2 {
                sum += nums.get(0).unwrap() * nums.get(1).unwrap();
            }
        }
    }
    sum
}
