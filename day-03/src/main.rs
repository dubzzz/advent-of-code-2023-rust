mod data;
mod part1;

fn main() {
    let input = data::get();

    let response = part1::run(input);
    println!("Response #1 is: {response}");
    assert_eq!(response, 527369);
}
