use std::io;
use std::collections::HashMap;

fn main() {
    // Read first line from the user
    let mut number_of_trips = String::new();
    io::stdin()
        .read_line(&mut number_of_trips)
        .expect("Failed to read n");

    let number_of_trips: i32 = number_of_trips.trim().parse().expect("Please enter a number");

    let mut trips: HashMap<String, Vec<i32>> = HashMap::new();

    for _i in 0..number_of_trips {
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Fail to read line");
        let mut split = line.trim().split_whitespace();
        let s: String = split.next().unwrap().to_string();
        let y: i32 = split.next().unwrap().parse().expect("Please type a number");

        trips.entry(s).or_insert_with(Vec::new).push(y)
    }

    // Sort all trip lists once
    for (_, years) in trips.iter_mut() {
        years.sort_unstable(); // Use sort_unstable which is faster
    }

    let mut number_of_queries = String::new();
    io::stdin()
        .read_line(&mut number_of_queries)
        .expect("Failed to read n");

    let number_of_queries: i32 = number_of_queries.trim().parse().expect("Please enter a number");

    for _i in 0..number_of_queries {
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Fail to read line");
        let mut split = line.trim().split_whitespace();
        let s: String = split.next().unwrap().to_string();
        let k: usize = split.next().unwrap().parse().expect("Please type a number");

        if let Some(ref mut vec) = trips.get_mut(&s) {
            println!("{}", vec[k-1])
        }
    }
}
