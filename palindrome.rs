use std::io;

fn main() {
    println!("Enter a string to check if it's a palindrome:");
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    let input = input.trim(); // Remove leading/trailing whitespace
    
    if is_palindrome(input) {
        println!("The string '{}' is a palindrome!", input);
    } else {
        println!("The string '{}' is not a palindrome.", input);
    }
}

fn is_palindrome(s: &str) -> bool {
    let reversed: String = s.chars().rev().collect();
    s == reversed
}
