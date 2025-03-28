use std::io;
fn main() {
    // Read first line from the user
    let mut first_line = String::new();
    io::stdin()
        .read_line(&mut first_line)
        .expect("Failed to read n");

    // Split the first line into 2 variables
    let mut split = first_line.trim().split_whitespace();
    let n: i32 = split.next().unwrap().parse().expect("Please type a number!");
    let t: i32 = split.next().unwrap().parse().expect("Please type a number!");
    
    // Read the next n lines from the users
    let mut line = String::new();
    let mut combined: Vec<(i32, i32)> = Vec::new();
    for _i in 0..n {
        line.clear();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read name");
        let mut split = line.trim().split_whitespace();
        let first = split.next().unwrap().parse().expect("Please type a number!");
        let second = split.next().unwrap().parse().expect("Please type a number!");
        combined.push((first, second));
    }
    
}
