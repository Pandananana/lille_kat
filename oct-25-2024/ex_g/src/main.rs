use std::collections::HashMap;
use std::io;

fn main() {
    // Get n
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read input");
    let n: i32 = n.trim().parse().expect("Please type a number!");

    // Save all inputs into one big vector
    let mut all_numbers = Vec::new();
    for _i in 0..10 * n {
        let mut line: String = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read input");
        let numbers: Vec<i32> = line
            .trim()
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().unwrap())
            .collect();
        all_numbers.extend(numbers);
    }

    let mut num_map = HashMap::new();
    for i in all_numbers {
        num_map
            .entry(i)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    let mut numbers_of_interest = Vec::new();
    for (key, value) in num_map.iter() {
        if *value > 2 * n {
            numbers_of_interest.push(*key);
        }
    }

    numbers_of_interest.sort();
    if numbers_of_interest.is_empty() {
        print!("{}", -1)
    } else {
        for i in numbers_of_interest {
            print!("{} ", i)
        }
    }
}
