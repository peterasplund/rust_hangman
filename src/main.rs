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
    std::process::Command::new("clear").status().expect("Couldn't clear line");
}

fn get_gfx(num_guesses: usize) -> String {
    let gfx = HANGMAN_PICS.get(num_guesses).expect("Out of bound when accessing gfx");
    gfx.to_string()
}

fn get_num_guesses(state: &State) -> usize {
    let mut i = 0;
    for g in &state.guesses {
        if !state.word.contains(g.to_owned()) {
            i += 1;
        }
    }

    return i;
}

fn main() {
    let mut input;
    let mut state = State { guesses: vec![], word: String::from("") };
    state.word = get_word();

    clear_terminal();
    println!("{}", "Welcome to hangman!".underline());

    loop {
        let num_guesses = get_num_guesses(&state);

        println!("{}", get_gfx(num_guesses).blue());
        println!("{}", get_board(&state));

        if num_guesses >= MAX_TRIES {
            println!("{}", "Game over!".red().bold());
            println!("The correct word was \"{}\"", state.word);
            break;
        }

        println!("Tries: {}/{}", num_guesses, MAX_TRIES);

        if has_won(&state) {
            println!("You won!");
            break;
        }

        println!("Enter a character");

        input = String::from("");
        io::stdin().read_line(&mut input).unwrap();

        clear_terminal();

        // Take null at end of string into consideration
        if input.len() < 1 || input == "\n" {
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

        if state.word.contains(guess) {
            println!("{}", "Nice one!".green());
        }
        else {
            println!("{}", "Not a match".red());
        }
    }
}
