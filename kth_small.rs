use std::io;

fn read_array() -> Vec<i32> {
    let mut input = String::new();
    println!("Enter the elements of the array separated by spaces:");

    io::stdin().read_line(&mut input).expect("Failed to read input");
    let numbers: Vec<i32> = input
        .split_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect();

    numbers
}

fn kth_smallest_element(arr: &[i32], k: usize) -> Option<i32> {
    if k == 0 || k > arr.len() {
        return None; // Invalid k value
    }
    
    let mut sorted_arr = arr.to_vec();
    sorted_arr.sort_by(|a, b| b.cmp(a)); // Sort in descending order

    Some(sorted_arr[k - 1])
}

fn main() {
    let arr = read_array();
    let k: usize;

    loop {
        println!("Enter the value of k:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        match input.trim().parse() {
            Ok(num) => {
                k = num;
                break;
            }
            Err(_) => println!("Please enter a valid number"),
        }
    }

    match kth_smallest_element(&arr, k) {
        Some(val) => println!("The {}th smallest element is: {}", k, val),
        None => println!("Invalid k value or array length"),
    }
}

