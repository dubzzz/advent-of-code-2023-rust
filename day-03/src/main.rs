mod data;
mod part1;
mod part2;

fn main() {
    let input = data::get();

    let response = part1::run(input);
    println!("Response #1 is: {response}");
    assert_eq!(response, 527369);

    let response = part2::run(input);
    println!("Response #2 is: {response}");
    assert_eq!(response, 73074886);
}
