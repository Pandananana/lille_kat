use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("Fail to read line");
    let mut split = line.trim().split_whitespace();
    let a: i32 = split.next().unwrap().parse().expect("Please type a number");
    let b: i32 = split.next().unwrap().parse().expect("Please type a number");
    let h: i32 = split.next().unwrap().parse().expect("Please type a number");

    let mut climbs = 0;
    let mut current_height = 0;
    let mut first_run = true;
    while current_height < h {
        if !first_run {
            current_height = current_height - b;
        }
        current_height = current_height + a;
        climbs = climbs + 1;
        first_run = false;
    }

    println!("{}", climbs);
}
