// Allows reading file
use std::fs;
use rand::seq::IteratorRandom;
use std::io::{self, Write};

// Max word length
const MWL: usize = 20;

fn main() {

    // Type: bool     Size: 26
    // Initializes with all false's
    let mut guessed: [bool; 26] = [false; 26];
    let mut word: [char; MWL] = ['\0'; MWL];
    let mut user: [char; MWL] = ['\0'; MWL];
    let mut word_size = 0;
    let mut guess: char = '\0';

    // Selecting word and filling dependent variables
    select_word(&mut word_size, &mut word, &mut user);
    user_input(&mut guess);
    /*
    while word != user {

    }
    */

    print_game_state(user, guessed, 2);
}

fn select_word(word_size: &mut i32, word: &mut [char; MWL], user: &mut [char; MWL]) {
    // Reading contents of file
    let bindings = fs::read_to_string("hangperson-words.txt")
        .expect("Should have been able to read the file");

    // Splitting contents by newline -> iterator object
    let contents = bindings.lines();

    // choose (part of IteratorRandom crate) selects a random element
    // from an iterator (among other array-types). thread_rng is a
    // random number generator that we gots to call. Unwrap is,
    // as usual extracting the value from a func that might return
    // None, and to_string converts from &str to String.
    let word_string = contents.choose(&mut rand::thread_rng()).unwrap().to_string();

    // filling arrays
    for chr in word_string.chars() {
        word[*word_size as usize] = chr;
        user[*word_size as usize] = '-';
        *word_size += 1;
    }
}

fn print_game_state(user: [char; MWL], guessed: [bool; 26], misses: i32) {

    // registering progress
    let mut body_parts: [char; 6] = ['o', '/', '|', '\\', '/', '\\'];
    for n in 0..6 {
        if n > misses {
            body_parts[n as usize] = ' ';
        }
    }

    // printing game board
    println!("\n|||----||       ");
    println!("|||  |          ");
    println!("|||  {0}          ", body_parts[0]);
    println!("|||  {0}{1}{2}        ", body_parts[1], body_parts[2], body_parts[3]);
    println!("|||  {0} {1}       ", body_parts[4], body_parts[5]);
    println!("|||             ");
    println!("|||=============\n");

    // printing state of hidden word
    for chr in user {
        print!("{}", chr);
    }

    // printing what the user has already guessed
    print!("\nAlready guessed: ");
    for n in 0..26 {
        if guessed[n as usize] {
            print!("{}", (n + 65) as u8 as char);
        }
    }
}

fn user_input(guess: &mut char) {

    // repeatedly taking string input from stdin
    // until it is a single alphabet character
    let mut string_in = String::new();
    let mut temp_char = '\0';
    loop {
        loop {
            print!("Enter your guess: ");
            io::stdout().flush().expect("flush failed!");
            io::stdin().read_line(&mut string_in).unwrap();
            if string_in.len() == 3 {
                break;
            }
            string_in = String::new();
        }
        temp_char = string_in.chars().next().expect("Something went wrong");
        if temp_char.is_alphabetic() {
            break;
        }
    }

    // updating guess pointer
    *guess = string_in.chars().next().expect("Something went wrong");

}
