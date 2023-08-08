use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please provide an API key");
        return;
    }

    let api_key = &args[1];
    println!("Using Open Weather Map API key: {}", api_key);
}
