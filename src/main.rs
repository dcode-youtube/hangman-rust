/* Subscribe to dcode on YouTube! */

/* Random Numbers */
extern crate rand;
use rand::Rng;

/* File Handling */
use std::fs::File;
use std::io::prelude::*;

/* User Input */
use std::io;

const ALLOWED_ATTEMPTS: u8 = 5;

struct Letter {
    character: char,
    revealed: bool
}

enum GameProgress {
    InProgress,
    Won,
    Lost
}

fn main() {
    let mut turns_left = ALLOWED_ATTEMPTS;
    let selected_word = select_word();
    let mut letters = create_letters(&selected_word);

    println!("Welcome to Hangman!");

    loop {
        println!("\nYou have {} turns left.", turns_left);
        display_progress(&letters);

        println!("\nPlease enter a letter to guess:");
        let user_char = read_user_input_character();

        /* Exit if user enters an asterisk ('*') */
        if user_char == '*' {
            break;
        }

        /* Update the 'revealed' state of each letter. If the user
        has guessed a correct letter, at_least_one_revealed is changed
        to true */
        let mut at_least_one_revealed = false;
        for letter in letters.iter_mut() {
            if letter.character == user_char {
                letter.revealed = true;
                at_least_one_revealed = true;
            }
        }

        /* Lose a turn if you make an incorrect guess */
        if !at_least_one_revealed {
            turns_left -= 1;
        }

        /* Check game progress */
        match check_progress(turns_left, &letters) {
            GameProgress::InProgress => continue,
            GameProgress::Won => {
                println!("\nCongrats! You won! ☺");
                break;
            }
            GameProgress::Lost => {
                println!("\nYou lost! ☹");
                break;
            }
        }
    }

    println!("\nGoodbye!");
}

/* Open the file containing list of words and select one at random, returning it
as a String */
fn select_word() -> String {
    let mut file = File::open("words.txt").expect("Could not open file!");

    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents).expect("An error occured while reading the file!");

    let available_words: Vec<&str> = file_contents.split(',').collect();

    /* Select word at random */
    let random_index = rand::thread_rng().gen_range(0, available_words.len());

    return String::from(available_words[random_index]);
}

/* Given a word (type String), create a Vector of Letter's from it with default
members and return it */
fn create_letters(word: &String) -> Vec<Letter> {
    let mut letters: Vec<Letter> = Vec::new();

    for c in word.chars() {
        letters.push(Letter {
            character: c,
            revealed: false
        });
    }

    return letters;
}

/* Displays the progress of the game based off Vec<Letter>
Example output: l _ n g _ _ g _ */
fn display_progress(letters: &Vec<Letter>) {
    let mut display_string = String::from("Progress:"); // Example: Progress: _ a _ a _ y

    for letter in letters {
        display_string.push(' ');

        if letter.revealed {
            display_string.push(letter.character);
        } else {
            display_string.push('_');
        }

        display_string.push(' ');
    }

    println!("{}", display_string);
}

/* Reads a character from user input. If multiple characters are given,
character at first index is returned. In any problematic cases, return
an asterisk (*) */
fn read_user_input_character() -> char {
    let mut user_input = String::new();

    match io::stdin().read_line(&mut user_input) {
        Ok(_) => {
            match user_input.chars().next() {
                Some(c) => { return c; }
                None => { return '*'; }
            }
        }
        Err(_) => { return '*'; }
    }
}

/* Checks the current state (progress) of the game and returns the appropriate
GameProgress member */
fn check_progress(turns_left: u8, letters: &Vec<Letter>) -> GameProgress {
    /* Determine if all letters have been revealed */
    let mut all_revealed = true;
    for letter in letters {
        if !letter.revealed {
            all_revealed = false;
        }
    }

    if all_revealed {
        return GameProgress::Won;
    }

    if turns_left > 0 {
        return GameProgress::InProgress;
    }

    return GameProgress::Lost;
}
