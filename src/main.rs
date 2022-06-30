use serde_json;
use std::collections::HashMap;
use std::{fs, io};
use itertools::Itertools;
use rand::seq::SliceRandom;

/**
Creates a new game of hangman
*/
fn new_game() -> Hangman {
    let secret = get_random_string();
    Hangman {
        answer: secret.clone(),
        user_answer: vec!["-".to_string(); secret.len()],
        failed_attempts: 0,
        guessed: String::from(""),
        alive: true,
        injured_last_turn: false,
    }
}

/**
The structure for hangman, hangman is alive until there are 5 failed attempts.
Failed attempts stored as i32.
User answer is stored as a vector of strings with each element default as "_".
Is alive and was injured last turn are booleans.
answer is a string.
 */
struct Hangman {
    answer: String,
    user_answer: Vec<String>,
    failed_attempts: i32,
    guessed: String,
    alive: bool,
    injured_last_turn: bool,
}
/**
"Implied" hangman
 */
impl Hangman {
    /**
    I LOVE GETTERS AND SETTERS
     */
    fn get_answer(&self) -> String { self.answer.clone() }
    fn get_user_answer(&self) -> String { self.user_answer.join("") }
    fn get_used_letters(&self) -> String { self.guessed.chars().sorted().collect::<String>() }
    fn get_failed_attempts(&self) -> i32 { self.failed_attempts }
    fn was_injured(&self) -> bool { self.injured_last_turn }

    /**
    Prints the current status of the game, used letters, bad letters, fails, hangman
     */
    fn print_game_status(&self) {
        println!("Phrase: {}", self.get_user_answer());
        println!("Used letters: {}", self.get_used_letters());
        println!("Failed attempts: {}", self.get_failed_attempts());
        if self.was_injured() {
            print_hangman(self.get_failed_attempts());
        }
    }

    /**
    Method for sorting if the guess was good or not.
    If guess is not ascii or already guessed,
     */
    fn sort_guess(&mut self, guess: String) {
        self.injured_last_turn = false;

        // Adds guess to list of guessed letters
        let guess= guess.to_lowercase();

        // There are no solutions with non ASCII characters
        if !&guess.is_ascii() {
            println!("Not an ASCII character. Not that it's a problem... \n\
             You just won't win with it...");
            return;
        }
        // If it has already been guessed
        else if self.get_used_letters().contains(&guess) {
            println!("Already guessed {}!", &guess);
            return
        } else {
            self.guessed.push_str(&guess);

        }

        // temp variables to store struct fields
        let mut pulled_user_answer = self.user_answer.clone();
        let pulled_answer = self.answer.clone();

        // If the guess is in the answer
        if pulled_answer.contains(&guess) {
            // i = index, c = char of answer at i
            for (i, c) in pulled_answer.chars().enumerate() {
                if guess == c.to_string() {
                    pulled_user_answer[i] = c.to_string();
                }
            }
            self.user_answer = pulled_user_answer;
        }
        // If the guess was a failure...
        else {
            self.failed_attempts += 1;
            self.injured_last_turn = true;
        }
    }

    /**
    Method for playing the game
     */
    fn play(&mut self) {
        let opening:String = "Welcome to Hangman!\n Let's play one quick game... \n".to_string();
        println!("{}", opening);

        while self.alive {
            println!("Next round - Please input your guess: ");
            let mut guess = String::new();

            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");
            guess = guess.trim().parse().unwrap();

            self.sort_guess(guess);
            // For winning the game
            if self.get_user_answer() == self.get_answer() {
                println!("You win! The answer was {}!", self.get_answer());
                break
            }
            // Stop playing if the hangman is hung. 5 is default for traditional sake
            if self.failed_attempts >= 5 {
                self.alive = false
            }
            self.print_game_status();
        }
        // For losing
        if !self.alive {
            println!("Game over!");
            println!("The answer was {}\n versus your answer: {}", self.get_answer(),
                     self.get_user_answer());

            // Numbers of missing charactes and number of unique characters
            let missing_characters = &self.get_user_answer().matches('-').count();
            let unique_characters = get_unique_characters(self.get_user_answer());

            println!("You were missing {} character(s) and {} unique character(s)",
                     missing_characters, unique_characters);
        }
    }
}

/**
Prints the hangman, depending on how many bad guesses,
Hangman will be upside down until the final bad guess
 */
fn print_hangman(bad_guesses: i32) {
    let hangman = "\
    +---------+
|   |    ,|
| , (),
|  \\|/
|   ^
|  | |     \n";

    let mut hangman_split = String::new();  // Hangman that can be complete or whole
    let _num_bad_guesses:usize = bad_guesses as usize;
    // Prints as much of the hangman as possible but upside down. One row per mistake
    if bad_guesses < 5 {
        for (f, line) in hangman.lines().rev().enumerate() {
            if f == _num_bad_guesses {
                break
            }
            hangman_split.push_str(line);
            hangman_split.push_str("\n");
        }
    }
    // If there were 5 guesses
    else {
        hangman_split.push_str(hangman);
    }
    println!("{}" ,hangman_split);
}

/**
Returns the total number of unique characters in a string
 */
fn get_unique_characters(strin: String) -> i32 {

    let letter_counts: HashMap<char, i32> = strin.chars()
        .fold(HashMap::new(), |mut map, c| {
        *map.entry(c).or_insert(0) += 1;
        map
    });

    *letter_counts.get(&'-').unwrap()
}

/**
Reads the words.json file and selects a random one for the hangman secret answer
 */
fn get_random_string() -> String {
    let file = fs::File::open("./src/words.json")
        .expect("file should open read only");
    let json: serde_json::Value = serde_json::from_reader(file)
        .expect("file should be proper JSON");
    let json_all_words = json.get("data")
        .expect("File not setup properly or corrupted");

    // Creates a vector based on the json
    let my_type: Vec<String> = serde_json::from_value(json_all_words.clone()).unwrap();

    // Randomly selects a word
    let mut rng = rand::thread_rng();
    let random_word = my_type.choose(&mut rng);
    let cleaned_random_word: String = random_word.unwrap().to_string();

    cleaned_random_word
}

/**
Let's play some hangman!
 */
fn main() {
    let mut game = new_game();
    game.play();
}