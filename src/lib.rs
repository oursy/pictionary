use rand::Rng;
use std::io;

/// Selects a random word from a predefined list.
///
/// # Returns
///
/// A randomly selected word from a static list of words.
pub fn select_word() -> &'static str {
    let words = vec!["apple", "banana", "cherry", "date", "elderberry"];
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..words.len());
    words[index]
}

/// Checks if the guessed word matches the secret word.
///
/// # Arguments
///
/// * `secret_word` - A string slice that holds the secret word.
/// * `guess` - A string slice that holds the user's guess.
///
/// # Returns
///
/// `true` if the guess matches the secret word, `false` otherwise.
pub fn check_guess(secret_word: &str, guess: &str) -> bool {
    secret_word == guess
}

/// Plays a round of the Pictionary game.
///
/// This function selects a secret word and prompts the user to guess it.
/// It continues to prompt for guesses until the user guesses correctly.
pub fn play_pictionary() {
    let secret_word = select_word();
    println!("Guess the word I am thinking of!");

    loop {
        let mut guess = String::new();
        println!("Please input your guess.");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess = guess.trim();

        if check_guess(secret_word, guess) {
            println!("You guessed correctly!");
            break;
        } else {
            println!("Try again.");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests that `check_guess` correctly identifies a correct guess.
    #[test]
    fn test_check_guess_correct() {
        assert!(check_guess("apple", "apple"));
    }

    /// Tests that `check_guess` correctly identifies an incorrect guess.
    #[test]
    fn test_check_guess_incorrect() {
        assert!(!check_guess("apple", "banana"));
    }

    /// Tests that `select_word` returns a word from the predefined list.
    #[test]
    fn test_select_word_is_in_list() {
        let word = select_word();
        let words = vec!["apple", "banana", "cherry", "date", "elderberry"];
        assert!(words.contains(&word));
    }
}
