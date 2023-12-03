use crate::parser;

pub fn run(corpus: &str) -> u32 {
    let games = corpus.split('\n').map(parser::parse_game);
    games
        .map(|parsed| {
            parsed
                .map(|game| {
                    if game
                        .turns
                        .iter()
                        .all(|turn| turn.red <= 12 && turn.green <= 13 && turn.blue <= 14)
                    {
                        game.id
                    } else {
                        0
                    }
                })
                .unwrap_or_default()
        })
        .sum()
}
