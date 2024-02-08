use std::io;

fn find_index(sorted_list: &[i32], target: i32) -> Option<usize> {
    let mut low = 0;
    let mut high = sorted_list.len() - 1;
    
    while low <= high {
        let mid = (low + high) / 2;
        if sorted_list[mid] == target {
            return Some(mid);
        } else if sorted_list[mid] < target {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }
    
    None
}

fn main() {
    // Read input from the user
    println!("Enter a sorted list of integers separated by spaces:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input.");
    
    // Parse the input into a vector of integers
    let sorted_list: Vec<i32> = input.trim().split_whitespace()
        .map(|s| s.parse().expect("Failed to parse input as integer."))
        .collect();
    
    // Use the first element as the target integer
    let target = sorted_list.first().cloned().unwrap_or(0);
    
    // Find the index of the target integer in the sorted list
    match find_index(&sorted_list, target) {
        Some(index) => println!("The {} is the Index of first occurrence number i.e {}", index, target),
        None => println!("{} not found in the sorted list.", target),
    }
}

