use std::env;

mod problems;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.get(1).map(|a| a.as_str()) {
        Some("1") => problems::p1::main(),
        Some("2") => problems::p2::main(),
        Some("3") => problems::p3::main(),
        _ => problems::p3::main(),
    }
}
