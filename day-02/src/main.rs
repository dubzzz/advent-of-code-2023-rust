mod data;
mod parser;

fn run1(corpus: &str) -> u32 {
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

fn run2(corpus: &str) -> u32 {
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

fn main() {
    let input = data::get();

    let response = run1(input);
    println!("Response #1 is: {response}");
    assert_eq!(response, 2369);

    let response = run2(input);
    println!("Response #2 is: {response}");
    assert_eq!(response, 66363);
}
