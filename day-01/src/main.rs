mod data;
mod part1;
mod part2_functional;
mod part2_imperative;

fn main() {
    let input = data::get();

    let response = part1::run(input);
    println!("Response #1 is: {response}");
    assert_eq!(response, 54304);

    let response = part2_functional::run(input);
    println!("Response #2 is: {response}");
    assert_eq!(response, 54418);

    let response = part2_imperative::run(input);
    println!("Response #2 is: {response}");
    assert_eq!(response, 54418);
}
