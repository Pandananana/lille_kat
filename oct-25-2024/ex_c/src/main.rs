use std::io;

fn main() {
    // Read a number from the user
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    print!("{}", input.chars().rev().collect::<String>())
}