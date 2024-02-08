use std::io;

fn reverse_string(input: &str) -> String {
    input.chars().rev().collect::<String>()
}

fn main() {
    println!("Enter a string:");

    let mut input_string = String::new();
    io::stdin().read_line(&mut input_string)
        .expect("Failed to read line");

    let reversed_string = reverse_string(input_string.trim());
    println!("Reversed string: {}", reversed_string);
}

