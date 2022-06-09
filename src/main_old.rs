use std::fs;
use std::io;
use rand::seq::SliceRandom;
use colored::*;

const MAX_TRIES: usize = 6;

const HANGMAN_PICS: &'static [&str] = &["
  +---+
  |   |
      |
      |
      |
      |
=========", "
  +---+
  |   |
  O   |
      |
      |
      |
=========", "
  +---+
  |   |
  O   |
  |   |
      |
      |
=========", "
  +---+
  |   |
  O   |
 /|   |
      |
      |
=========", "
  +---+
  |   |
  O   |
 /|\\  |
      |
      |
=========", "
  +---+
  |   |
  O   |
 /|\\  |
 /    |
      |
=========", "
  +---+
  |   |
  O   |
 /|\\  |
 / \\  |
      |
========="];

struct State {
    guesses: Vec<char>,
    word: String,
}

fn get_word() -> String {
    let file: String = fs::read_to_string("text.txt").expect("Could not read file");

    let replaced_file: String = file.replace("\n", " ");

    let lines: Vec<&str> = replaced_file.split(" ").filter(|&x| x != "").collect();

    match lines.choose(&mut rand::thread_rng()) {
        Some(l) => return l.to_string(),
        None => {
            panic!("faulty word");
        }
    };
}

fn get_board(state: &State) -> String {
    let mut s = String::from("");

    for c in state.word.chars() {
        if state.guesses.contains(&c) {
            s.push_str(&c.to_string());
        }
        else {
            s.push_str("_");
        }

    }

    return s
}

fn get_correct_guesses(state: &State) -> usize {
    let mut num = 0;
    for c in state.word.chars() {
        if state.guesses.contains(&c) {
            num += 1;
        }
    }

    return num;
}

fn has_won(state: &State) -> bool {
    let correct_guesses = get_correct_guesses(&state);
    return correct_guesses == state.word.len();
}

fn clear_terminal() {
    print!("{}[2J", 27 as char);
}

fn print_gfx(state: &State) {
    let gfx = HANGMAN_PICS.get(state.guesses.len()).expect("Out of bound when accessing gfx");

    println!("{}", gfx.blue());
}

fn main() {
    let mut input;
    let mut state = State { guesses: vec![], word: String::from("") };
    state.word = get_word();

    clear_terminal();

    loop {
        print_gfx(&state);
        println!("{}", get_board(&state));

        if state.guesses.len() >= MAX_TRIES {
            println!("{}", "Game over!".red().bold());
            break;
        }

        println!("Tries: {}/{}", state.guesses.len(), MAX_TRIES);

        if has_won(&state) {
            println!("You won!");
            break;
        }

        println!("Enter a character");

        input = String::from("");
        io::stdin().read_line(&mut input).unwrap();

        // Take null at end of string into consideration
        if input.len() < 1 {
            println!("{}", "Please enter a character".yellow());
            continue;
        }
        if input.len() > 2 {
            println!("{}", "Please enter one charater only".yellow());
            continue;
        }

        let guess = input.chars().nth(0).unwrap();

        if state.guesses.contains(&guess) {
            println!("{}", "You've already guessed this".yellow());
            continue;
        }

        state.guesses.push(guess);

        clear_terminal();

        if state.word.contains(guess) {
            println!("{}", "Nice one!".green());
        }
        else {
            println!("{}", "Not a match".red());
        }
    }
}
