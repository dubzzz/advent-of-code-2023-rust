mod data;
mod functional;
mod imperative;

fn main() {
    let input = data::get();

    let response = functional::run(input);
    println!("Response is: {response}");
    assert_eq!(response, 54418);

    let response = imperative::run(input);
    println!("Response is: {response}");
    assert_eq!(response, 54418);
}
