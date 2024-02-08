use std::io;

fn max_subarray_sum(arr: &[i32]) -> i32 {
    let mut max_ending_here = arr[0];
    let mut max_so_far = arr[0];

    for &num in arr.iter().skip(1) {
        max_ending_here = num.max(max_ending_here + num);
        max_so_far = max_so_far.max(max_ending_here);
    }

    max_so_far
}

fn main() {
    // Read array size from the user
    println!("Enter the size of the array:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let size: usize = input.trim().parse().expect("Please enter a valid number");

    // Read array elements from the user
    println!("Enter the array elements separated by space:");
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let arr: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Please enter valid integers"))
        .collect();

    // Check if array size matches the number of elements entered
    if arr.len() != size {
        println!("Error: Array size does not match the number of elements entered");
        return;
    }

    // Find maximum subarray sum and print it
    let max_sum = max_subarray_sum(&arr);
    println!("Maximum subarray sum is: {}", max_sum);
}

