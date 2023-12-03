use crate::parser;

pub fn run(corpus: &str) -> u32 {
    let games = corpus.split('\n').map(parser::parse_game);
    games
        .filter_map(|parsed| parsed.ok())
        .filter(|game| {
            game.turns
                .iter()
                .all(|turn| turn.red <= 12 && turn.green <= 13 && turn.blue <= 14)
        })
        .map(|game| game.id)
        .sum()
}
