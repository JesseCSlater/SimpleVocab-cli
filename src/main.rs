use std::io;
use std::io::Write;

fn main() {
    println!("Welcome to SimpleVocab!");
    let test_word: Word = Word {
        word: String::from("Hello"),
        synonyms: Vec::from([String::from("Hi"), String::from("Greetings")]),
        definition: String::from("Used for greeting."),
    };
    test(test_word);
    match select_option(&["zero", "one", "two"]) {
        Ok(n) => {
            println!("{}", n);
        }
        Err(e) => {
            println!("{e}");
        }
    }
}

struct Word {
    word: String,
    synonyms: Vec<String>,
    definition: String,
}

fn test(w: Word) {
    println!("{}", w.word);
}

fn select_option(options: &[&str]) -> Result<usize, io::Error> {
    for (i, option) in options.into_iter().enumerate() {
        println!("{i}) {option}");
    }
    print!(">");
    io::stdout().flush()?;

    let mut input: String;
    loop {
        input = String::new();
        io::stdin().read_line(&mut input)?;
        match input.trim().parse::<usize>() {
            Ok(n) => {
                return Ok(n);
            }
            Err(_) => {
                println!("Invalid input {}. Input one of the options", input.trim());
                print!(">");
                io::stdout().flush()?;
            }
        }
    }
}
