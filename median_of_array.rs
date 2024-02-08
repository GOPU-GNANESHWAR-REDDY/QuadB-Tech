use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter the elements of the array separated by spaces:");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let mut numbers: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Not a number!"))
        .collect();

    numbers.sort();

    let middle_index = numbers.len() / 2;

    let middle_value = numbers[middle_index];

    println!("Middle value of the sorted array: {}", middle_value);
}

