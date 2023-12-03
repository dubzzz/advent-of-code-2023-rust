mod data;

#[derive(Clone)]
struct ParsedTurn {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Clone)]
struct ParsedGame {
    id: u32,
    turns: Vec<ParsedTurn>,
}

fn parse_turn(turn_str: &str) -> Result<ParsedTurn, &str> {
    let empty_parsed = ParsedTurn {
        red: 0,
        green: 0,
        blue: 0,
    };
    turn_str.split(',').map(|member| member.trim()).fold(
        Ok(empty_parsed),
        |acc, member| match acc {
            Ok(parsed) => match member.find(' ') {
                None => Err("Unable to find the end of the count in the member"),
                Some(count_end) => match &member[..count_end].parse::<u32>() {
                    Err(_) => Err("Unable to parse the count as numeric"),
                    Ok(count) => match &member[(count_end + 1)..] {
                        "red" => Ok(ParsedTurn {
                            red: *count,
                            ..parsed
                        }),
                        "green" => Ok(ParsedTurn {
                            green: *count,
                            ..parsed
                        }),
                        "blue" => Ok(ParsedTurn {
                            blue: *count,
                            ..parsed
                        }),
                        _ => Err("Non-existing category"),
                    },
                },
            },
            other => other,
        },
    )
}

fn parse_game(game_str: &str) -> Result<ParsedGame, &str> {
    if !game_str.starts_with("Game ") {
        return Err("Not starting by Game");
    }
    let game_id_end = game_str.find(':');
    if game_id_end.is_none() {
        return Err("Not containing the ':'");
    }
    let game_id_end = game_id_end.unwrap();
    let game_id = &game_str[5..game_id_end];
    let game_id = game_id.parse::<u32>();
    if game_id.is_err() {
        return Err("Invalid game id");
    }
    let game_id = game_id.unwrap();
    let turns: &Vec<ParsedTurn> = &game_str[(game_id_end + 1)..]
        .split(';')
        .map(|turn_str| parse_turn(turn_str).unwrap())
        .collect();
    let parsed = ParsedGame {
        id: game_id,
        turns: turns.to_vec(),
    };
    return Ok(parsed);
}

fn run(corpus: &str) -> u32 {
    let games = corpus.split('\n').map(parse_game);
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

fn main() {
    let input = data::get();

    let response = run(input);
    println!("Response is: {response}");
    assert_eq!(response, 2369);
}
