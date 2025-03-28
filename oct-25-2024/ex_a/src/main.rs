use std::io;

fn main() {
    // Read a number from the user
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read n");

    let n: i32 = n.trim().parse().expect("Please type a number!");

    for _i in 0..n {
        // Read a number from the user
        let mut name = String::new();
        io::stdin()
            .read_line(&mut name)
            .expect("Failed to read name");

        print!("Takk {}", name)
    }
}