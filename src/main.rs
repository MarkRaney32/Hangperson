// Allows reading file
use std::fs;
use rand::seq::IteratorRandom;
use std::io::{self, Write};
//use std::ascii::AsciiExt;

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
    let mut misses = 0;

    // Selecting word and filling dependent variables
    select_word(&mut word_size, &mut word, &mut user);
    print_game_state(user, guessed, misses);
    user_input(&mut guess, &mut guessed);
    misses += update_game_state(guess, word_size, word, &mut user);

    // game loop
    while word != user {
        if misses > 5 { break; }
        print_game_state(user, guessed, misses);
        user_input(&mut guess, &mut guessed);
        misses += update_game_state(guess, word_size, word, &mut user);
    }

    // Printing end condition
    if misses > 5 {
        print!("\nYou lose! The word was: ");
        for chr in word {
            if chr == '\0' {break;}
            print!("{}", chr);
        }
    } else {
        println!("\nYou win!");
    }

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
        if n >= misses {
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

    println!("\n\nMisses: {}", misses);

    // printing what the user has already guessed
    print!("Already guessed: ");
    for n in 0..26 {
        if guessed[n as usize] {
            print!("{}", (n + 65) as u8 as char);
        }
    }
}

fn user_input(guess: &mut char, guessed: &mut [bool; 26]) {

    // repeatedly taking string input from stdin
    // until it is a single alphabet character
    let mut string_in = String::new();
    let mut temp_char;
    loop {
        loop {
            print!("\nEnter your guess: ");
            io::stdout().flush().expect("flush failed!");
            io::stdin().read_line(&mut string_in).unwrap();
            if string_in.len() == 3 {
                break;
            }
            string_in = String::new();
        }
        temp_char = string_in.chars().next().expect("Something went wrong");
        if temp_char.is_alphabetic() && !already_guessed(temp_char.to_ascii_uppercase(), guessed){
            break;
        }
    }

    *guess = temp_char.to_ascii_uppercase();

}

fn already_guessed(guess: char, guessed: &mut [bool; 26]) -> bool {
    // Checks if the user input has already been entered
    if guessed[guess as usize - 65] {
        return true;
    }
    guessed[guess as usize - 65] = true;
    return false;
}

fn update_game_state(guess: char, word_size: i32, word: [char; MWL], user: &mut [char; MWL]) -> i32{
    // Updates current user game state
    // returns correctness of guess:
    //     0: correct guess
    //     1: incorrect guess

    let mut result = 1;
    for index in 0..word_size {
        if guess == word[index as usize].to_ascii_uppercase() {
            user[index as usize] = guess;
            result = 0;
        }
    } return result;
}
