use std::io;

fn longest_common_prefix(strings: &[String]) -> String {
    if strings.is_empty() {
        return String::new();
    }

    let first_str = &strings[0];
    let mut prefix_length = first_str.len();

    for other_str in strings.iter().skip(1) {
        prefix_length = prefix_length.min(other_str.len());
        for (i, byte) in first_str.as_bytes().iter().zip(other_str.as_bytes().iter()).enumerate() {
            if byte != (&first_str.as_bytes()[i], &other_str.as_bytes()[i]) {
                prefix_length = i;
                break;
            }
        }
    }

    String::from(&first_str[..prefix_length])
}

fn main() {
    let mut input_strings: Vec<String> = Vec::new();

    println!("Enter strings (press Enter after each string, type 'done' when finished):");

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let input_trimmed = input.trim();

        if input_trimmed == "done" {
            break;
        }

        input_strings.push(String::from(input_trimmed));
    }

    let common_prefix = longest_common_prefix(&input_strings);

    println!("Longest common prefix: {}", common_prefix);
}

