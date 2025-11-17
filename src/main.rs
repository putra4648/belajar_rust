use std::string;

pub mod calculator;

fn main() {
    let add = calculator::calculate(vec![1.0, 2.0, 3.0, 4.0, 5.0], string::String::from("+"));
    println!("Addition Result: {}", add);

    let multiply = calculator::calculate(vec![1.0, 2.0, 3.0, 4.0, 5.0], string::String::from("*"));
    println!("Multiplication Result: {}", multiply);

    let divide = calculator::calculate(vec![100.0, 2.0], string::String::from("/"));
    println!("Division Result: {}", divide);

    let subtract = calculator::calculate(vec![100.0, 50.0, 25.0], string::String::from("-"));
    println!("Subtraction Result: {}", subtract);
}
