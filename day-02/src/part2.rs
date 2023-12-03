use crate::parser;

pub fn run(corpus: &str) -> u32 {
    let games = corpus.split('\n').map(parser::parse_game);
    games
        .map(|parsed| -> u32 {
            let parsed = parsed.unwrap();
            let max_reds = parsed.turns.iter().map(|t| t.red).max().unwrap_or_default();
            let max_greens = parsed
                .turns
                .iter()
                .map(|t| t.green)
                .max()
                .unwrap_or_default();
            let max_blues = parsed
                .turns
                .iter()
                .map(|t| t.blue)
                .max()
                .unwrap_or_default();
            max_reds * max_greens * max_blues
        })
        .sum()
}
