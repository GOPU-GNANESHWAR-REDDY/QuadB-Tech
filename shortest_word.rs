use std::io;

fn shortest_words(sentence: &str) -> Vec<&str> {
    let mut shortest = Vec::new();
    let mut shortest_len = usize::MAX;

    for word in sentence.split_whitespace() {
        let word_len = word.len();
        if word_len < shortest_len {
            shortest.clear();
            shortest.push(word);
            shortest_len = word_len;
        } else if word_len == shortest_len {
            shortest.push(word);
        }
    }

    shortest
}

fn main() {
    println!("Please enter a sentence:");

    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    let shortest = shortest_words(&input);

    if shortest.is_empty() {
        println!("No words found in the input.");
    } else {
        println!("The shortest word(s) is/are:");
        for word in shortest {
            println!("{}", word);
        }
    }
}

