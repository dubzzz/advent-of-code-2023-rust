use crate::parser;

pub fn run(corpus: &str) -> u32 {
    corpus
        .split('\n')
        .map(parser::parse_game)
        .filter_map(|parsed| parsed.ok())
        .map(|game| -> u32 {
            let max_reds = game.turns.iter().map(|t| t.red).max().unwrap_or_default();
            let max_greens = game.turns.iter().map(|t| t.green).max().unwrap_or_default();
            let max_blues = game.turns.iter().map(|t| t.blue).max().unwrap_or_default();
            max_reds * max_greens * max_blues
        })
        .sum()
}
