// Simple Hangman Program
// User gets five incorrect guesses
// Word chosen randomly from words.txt
// Inspiration from: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
// This assignment will introduce you to some fundamental syntax in Rust:
// - variable declaration
// - string manipulation
// - conditional statements
// - loops
// - vectors
// - files
// - user input
// We've tried to limit/hide Rust's quirks since we'll discuss those details
// more in depth in the coming lectures.
extern crate rand;
use rand::prelude::SliceRandom;
use rand::Rng;
use std::fs;
use std::io;
use std::io::Write;

const NUM_INCORRECT_GUESSES: u32 = 5;
const WORDS_PATH: &str = "words.txt";

fn pick_a_random_word() -> String {
    let file_string = fs::read_to_string(WORDS_PATH).expect("Unable to read file.");
    let words: Vec<&str> = file_string.split('\n').collect();
    String::from(words[rand::thread_rng().gen_range(0, words.len())].trim())
}

#[derive(Debug)]
struct GameState {
    board: Vec<LetterState>,
    secret_word: String,
    secret_words_chars: Vec<char>,
    remaining_guesses: u32,
    correct_guesses: Vec<char>,
}

#[derive(Clone, Copy, Debug)]
enum LetterState {
    Guessed,
    NotGuessed,
}

#[derive(Clone, Debug)]
enum GuessResult {
    Correct,
    Incorrect,
}

impl GameState {
    pub fn new(secret_word: String) -> GameState {
        GameState {
            board: vec![LetterState::NotGuessed; secret_word.len()],
            secret_word: secret_word.clone(),
            secret_words_chars: secret_word.chars().collect(),
            remaining_guesses: NUM_INCORRECT_GUESSES,
            correct_guesses: Vec::new(),
        }
    }

    fn filter_by_letterstate(&self, guessed: bool) -> Vec<char> {
        self.secret_words_chars
            .iter()
            .enumerate()
            .filter(|(ix, _)| match self.board[*ix] {
                LetterState::Guessed => guessed,
                LetterState::NotGuessed => !guessed,
            })
            .map(|(_, l)| *l)
            .collect()
    }

    fn unguessed_letters(&self) -> Vec<char> {
        self.filter_by_letterstate(false)
    }

    fn guess(&mut self, c: &char) -> GuessResult {
        if self.unguessed_letters().contains(c) {
            println!("");
            self.correct_guesses.push(*c);

            let mut match_ixs: Vec<usize> = self
                .secret_words_chars
                .iter()
                .enumerate()
                .filter_map(|(ix, l)| if *l == *c { Some(ix) } else { None })
                .collect();

            if match_ixs.len() > 1 {
                let mut rng = rand::thread_rng();
                match_ixs.shuffle(&mut rng);
            }

            let ix_to_change: usize = match_ixs
                .iter()
                .skip_while(|ix| match self.board[**ix] {
                    LetterState::Guessed => true,
                    LetterState::NotGuessed => false,
                })
                .cloned()
                .take(1)
                .next()
                .unwrap();

            self.board[ix_to_change] = LetterState::Guessed;
            GuessResult::Correct
        } else {
            println!("Sorry, that letter is not in the word");
            println!("");
            self.remaining_guesses -= 1;
            GuessResult::Incorrect
        }
    }

    pub fn print_status(&self) {
        println!(
            "\
The word so far is {}
You have guessed the following letters: {}
You have {} guesses left",
            self.redacted_board(),
            self.correct_guesses.iter().collect::<String>(),
            self.remaining_guesses
        );
    }

    fn redacted_board(&self) -> String {
        self.secret_words_chars
            .iter()
            .enumerate()
            .map(|(ix, l)| match self.board[ix] {
                LetterState::Guessed => l,
                LetterState::NotGuessed => &'-',
            })
            .collect()
    }
}

fn main() {
    println!("Welcome to CS110L Hangman!");
    let mut game_state = GameState::new(pick_a_random_word());
    println!("secret word: {}", game_state.secret_word);
    game_state.print_status();

    // main game loop
    loop {
        if game_state.remaining_guesses <= 0 {
            println!("");
            println!("Sorry, you ran out of guesses!");
            break;
        }

        if game_state.unguessed_letters().is_empty() {
            println!("");
            println!(
                "Congratulations you guessed the secret word: {}!",
                game_state.secret_word
            );
            break;
        }

        print!("Please guess a letter: ");
        // Make sure the prompt from the previous line gets displayed:
        io::stdout().flush().expect("Error flushing stdout.");
        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Error reading line.");

        let letters: Vec<char> = user_input.chars().filter(|c| c.is_alphabetic()).collect();

        if letters.is_empty() {
            println!("I didn't get any letters!");
            continue;
        };

        // at this point we know we have a vec of letters of length >= 1, the unwrap on nth is (probably) ok here.
        let guessed_letter = letters.iter().nth(0).unwrap();

        game_state.guess(guessed_letter);
        game_state.print_status();
    }
}
