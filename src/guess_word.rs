/*
* Crea un pequeño juego que consista en adivinar palabras en un número máximo de intentos:
* - El juego comienza proponiendo una palabra aleatoria incompleta y el número de intentos.
* - El usuario puede introducir únicamente una letra o una palabra (de la misma longitud que la
*   palabra a adivinar)
* - Si escribe una letra y acierta, se muestra esa letra en la palabra. Si falla, se resta
*   uno al número de intentos
* - Si escribe una resolución y acierta, finaliza el juego, en caso contrario, se resta uno
*   al número de intentos
* - Si el contador de intentos llega a 0, el jugador pierde
* - La palabra debe ocultar de forma aleatoria letras, y nunca puede comenzar ocultando más del 60%
* - Puedes utilizar las palabras que quieras y el número de intentos que consideres
*/

use fakeit::words;
use rand::{thread_rng, Rng};
use std::{io, mem};

pub fn guess_word_main() {
    let mut attempts = 4;
    // Generate random word to guess
    let mut word = String::new();
    while word.len() < 4 {
        word = words::word();
    }
    // Turn word to vec for better manipulation
    let word_as_vec: Vec<char> = word.chars().collect();

    // Get random index of letters to hide in the original word
    let mut hidden_indexes = hide_indexes(&word_as_vec);

    // Create string to store user input
    let mut guess = String::new();

    loop {
        if attempts <= 0 {
            println!("You have no more attempts, you lost.\nThe word was: {word}");
            break;
        }

        if hidden_indexes.len() == 0 {
            println!("You win! The word is: {word}");
            break;
        }

        println!(
            "The word is: {}\nAttempts remaining: {attempts}",
            hide_word_by_index(&word_as_vec, &hidden_indexes)
        );

        // Get user input
        guess.clear();
        println!("Your input:");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");
        guess = guess.trim().to_string();

        if guess.len() == word.len() {
            if guess.eq(&word) {
                println!("You win!: The word is {word}");
                break;
            } else {
                println!("Wrong word.");
                attempts -= 1;
                continue;
            }
        } else if guess.len() == 1 {
            let guess = guess.chars().next().unwrap();
            let mut founded_indexes: Vec<u8> = Vec::new();

            for i in &hidden_indexes {
                if word_as_vec[*i as usize] == guess {
                    founded_indexes.push(*i);
                }
            }

            if founded_indexes.len() == 0 {
                println!("Wrong letter.");
                attempts -= 1;
                continue;
            } else {
                for i in &founded_indexes {
                    let index = hidden_indexes.iter().position(|x| x == i).unwrap();
                    hidden_indexes.remove(index);
                }
                println!("Correct letter.");
                continue;
            }
        } else {
            println!("Please, input just one letter or the entire word.");
            continue;
        }
    }
}

// Return a list of random indexes to hide in a word
fn hide_indexes(word: &Vec<char>) -> Vec<u8> {
    let max_hidden_letters = word.len() / 3;
    let mut hidden_indexes: Vec<u8> = Vec::new();

    for _ in 0..max_hidden_letters {
        let index: u8 = thread_rng().gen_range(0..word.len() as u8);
        hidden_indexes.push(index);
    }
    hidden_indexes
}

// Return a word with hidden letters according an index list
fn hide_word_by_index(word: &Vec<char>, indexes: &Vec<u8>) -> String {
    let mut word_as_vec = word.to_owned();
    for i in indexes {
        let _ = mem::replace(&mut word_as_vec[*i as usize], '_');
    }
    word_as_vec.iter().collect::<String>()
}
