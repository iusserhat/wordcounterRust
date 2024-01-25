struct WordCounter {
    text: String,
}

impl WordCounter {
    fn new(text: &str) -> WordCounter {
        WordCounter {
            text: text.to_string(),
        }
    }

    fn count_words(&self) -> Result<usize, &'static str> {
        if self.text.is_empty() {
            return Err("Text is empty");
        }

        let words: Vec<&str> = self.text.split_whitespace().collect();
        Ok(words.len())
    }
}

fn main() {
    println!("Enter a text:");
    let mut text = String::new();
    std::io::stdin().read_line(&mut text).expect("Failed to read input");

    let counter = WordCounter::new(&text.trim());

    match counter.count_words() {
        Ok(word_count) => println!("Word count: {}", word_count),
        Err(error_message) => println!("Error: {}", error_message),
    }
}
