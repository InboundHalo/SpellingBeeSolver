use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::env;
use std::env::current_dir;
use std::time::Instant;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 8 {
        eprintln!("Usage: {} <L> <A> <B> <C> <D> <E> <F>", args[0]);
        eprintln!("Example: {} a b c d e f g", args[0]);
        return Ok(());
    }

    let letters: Vec<char> = args[1..8].iter().map(|string: &String| {
        // make sure all chars are alphabetic and lowercase
        let character :char = string.chars().next().unwrap();
        if !character.is_ascii_alphabetic() {
            panic!("All inputs must be alphabetic characters.");
        }
        character.to_ascii_lowercase()
    }).collect();

    let specified_letter = letters[0];
    let allowed_letters: Vec<char> = letters.iter().cloned().collect();

    let now = Instant::now();

    println!("{}", current_dir().unwrap().to_str().unwrap());

    let likely_valid_words = find_words_from_file(
        get_reader_from_path("src\\mitwordlist10000.txt"),
        specified_letter,
        &allowed_letters
    );

    let all_valid_words = find_words_from_file(
        get_reader_from_path("src\\mitwordlist100000.txt"),
        specified_letter,
        &allowed_letters
    );

    let all_perfect_valid_words = find_perfect_words_from_file(
        &all_valid_words,
        &allowed_letters
    );

    let elapsed = now.elapsed();
    println!("Elapsed: {:.5?}", elapsed);

    // 100000 + 100000 + 10000 = 210,000

    println!("Likely words: ");
    for word in &likely_valid_words {
        println!("{}", word);
    }

    println!();
    println!("All words: ");
    for word in &all_valid_words {
        println!("{}", word);
    }

    println!();
    println!("All Perfect words: ");
    for word in &all_perfect_valid_words {
        println!("{}", word);
    }

    Ok(())
}

fn get_reader_from_path(path: &str) -> BufReader<File> {
    if !Path::new(path).exists() {
        eprintln!("Dictionary file not found at {}", path);
        panic!()
    }

    let file= File::open(path).unwrap();
    BufReader::new(file)
}

fn find_words_from_file(reader: BufReader<File>, specified_letter: char, allowed_letters: &Vec<char>) -> Vec<String> {
    let mut final_words: Vec<String> = Vec::new();

    for line in reader.lines() {
        let word: String = line.unwrap().trim().to_lowercase();

        // Requirements:
        // 4 letter or more word
        // contain the specified_letter
        // only contain letters in allowed_letters

        if word.len() < 4 {
            continue;
        }

        if !word.chars().all(|c| allowed_letters.contains(&c)) {
            continue;
        }

        if !word.contains(specified_letter) {
            continue;
        }

        final_words.push(word);
    }

    final_words.sort_by(|a, b| b.len().cmp(&a.len()));

    final_words
}

fn find_perfect_words_from_file(valid_words : &Vec<String>, allowed_letters: &Vec<char>) -> Vec<String> {
    let mut final_words: Vec<String> = Vec::new();

    for line in valid_words {
        let word: String = line.trim().to_lowercase();

        // Requirements:
        // contain all the allowed_letters

        if word.len() < allowed_letters.len() {
            continue;
        }

        let mut valid = true;

        for character in allowed_letters {
            if !word.contains(*character) {
                valid = false;
            }
        }

        if !valid {
            continue;
        }

        final_words.push(word);
    }

    final_words.sort_by(|a, b| b.len().cmp(&a.len()));

    final_words
}
