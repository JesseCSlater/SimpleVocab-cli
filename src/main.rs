#![feature(let_chains)]
use std::io;
use std::io::Write;

fn main() {
    println!("Welcome to SimpleVocab!");
    let hello: Word = Word {
        word: String::from("Hello"),
        synonyms: Vec::from([String::from("Hi"), String::from("Greetings")]),
        definition: String::from("Used for greeting."),
    };
    test(hello);
    println!("{}", select_option(&["zero", "one", "two"]))
}

struct Word {
    word: String,
    synonyms: Vec<String>,
    definition: String,
}

fn test(w: Word) {
    println!("{}", w.word);
}

fn select_option(options: &[&str]) -> usize {
    for (i, option) in options.into_iter().enumerate() {
        println!("{i}) {option}");
    }

    let mut input: String = String::new();
    loop {
        print!(">");
        io::stdout().flush().unwrap();
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let n = input.trim().parse::<usize>();
        if let Ok(i) = n && i < options.len() {
            return i
        }
        eprintln!(
            "Invalid input {}. Please select one of the options.",
            input.trim()
        );
    }
}
