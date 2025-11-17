
pub fn calculate(args: Vec<f32>, symbol: String) -> f32 {
    match symbol.as_str() {
        "+" => args.iter().fold(0.0, |acc, &x| acc + x),
        "*" => args.iter().fold(1.0, |acc, &x| acc * x),
        "/" => args.iter().copied().reduce(|a, x| a / x).unwrap_or(0.0),
        "-" => args.iter().copied().reduce(|a, x| a - x).unwrap_or(0.0),
        _ => panic!("Unsupported operation"),
    }
    
}
