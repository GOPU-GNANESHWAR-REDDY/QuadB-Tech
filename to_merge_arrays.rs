use std::io;

fn main() {
    // Function to read an array from the user
    fn read_array() -> Vec<i32> {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        input.trim().split_whitespace()
            .map(|s| s.parse().expect("Invalid input"))
            .collect()
    }

    // Read the first array
    println!("Enter the first array (space-separated):");
    let mut array1 = read_array();

    // Read the second array
    println!("Enter the second array (space-separated):");
    let array2 = read_array();

    // Merge the arrays
    array1.extend(array2);

    // Print the merged array
    println!("Merged Array: {:?}", array1);

    // Ask the user if they want to sort the array
    println!("Do you want to sort the array? (yes/no)");
    let mut response = String::new();
    io::stdin().read_line(&mut response).expect("Failed to read line");

    if response.trim().to_lowercase() == "yes" {
        // Sort and print the array
        array1.sort();
        println!("Sorted Array: {:?}", array1);
    }
}

