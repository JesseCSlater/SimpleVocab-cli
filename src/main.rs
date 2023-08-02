#![feature(let_chains)]
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io;
use std::io::Write;

fn main() {
    println!("Welcome to SimpleVocab!");
    let mut hello: Word = Word {
        word: String::from("hello"),
        synonym: String::from("hi, greetings"),
        definition: String::from("Used for greeting."),
        test_history: Vec::new(),
    };
    test(&mut hello);
}

#[derive(Serialize, Deserialize, Debug)]
struct Word {
    word: String,
    synonym: String,
    definition: String,
    test_history: Vec<(DateTime<Utc>, TestResult)>,
}

#[derive(Serialize, Deserialize, Debug)]
enum TestResult {
    Know,
    DontKnow,
}

fn test(w: &mut Word) {
    println!("{}", w.word);
    loop {
        match select_option(&[
            "Know",
            "Don't know",
            "Show synonyms",
            "Show definition",
            "Back",
        ]) {
            0 => {
                w.test_history.push((Utc::now(), TestResult::Know));
                break;
            }
            1 => {
                w.test_history.push((Utc::now(), TestResult::DontKnow));
                break;
            }
            2 => println!("{}", w.synonym),
            3 => println!("{}", w.definition),
            4 => return,
            _ => return,
        }
    }
    println!("{}", serde_json::to_string(&w).unwrap());
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
        if let Ok(i) = input.trim().parse::<usize>() && i < options.len() {
            return i
        }
        eprintln!(
            "Invalid input {}. Please select one of the options.",
            input.trim()
        );
    }
}
