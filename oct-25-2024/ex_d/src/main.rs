use std::io;

fn main() {
    let mut line_1 = String::new();
    io::stdin()
        .read_line(&mut line_1)
        .expect("Failed to read input");

    // Read a number from the user
    let mut line_2 = String::new();
    io::stdin()
        .read_line(&mut line_2)
        .expect("Failed to read input");
    let line_2 = line_2.trim().split(" ").collect::<Vec<_>>();

    // Read a number from the user
    let mut line_3 = String::new();
    io::stdin()
        .read_line(&mut line_3)
        .expect("Failed to read input");
    let line_3 = line_3.trim().split(" ").collect::<Vec<_>>();

    for num in line_2 {
        if line_3.contains(&num) {
            print!("{} ",num)
        }
    }
}