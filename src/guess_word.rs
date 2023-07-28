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

pub fn guess_word() {
    let mut word = String::from("");
    while word.len() < 4 {
        word = words::word();
    }
    println!("Word to guess: {word}");

    let mut guess = String::new();

    loop {
        println!("Please, input your guess.");
        guess.clear();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        guess = guess.trim().to_string();

        if word.eq(&guess) {
            println!("You win!");
            break;
        } else {
            continue;
        }
    }
}

fn hide_word(word: &str) -> String {
    let mut word: Vec<char> = word.chars().collect();
    let word_len = word.len();
    let max_hidden_letters = word_len / 3;

    let mut rand_indexes: Vec<u8> = Vec::new();

    for _ in 0..max_hidden_letters {
        let index: u8 = thread_rng().gen_range(0..word_len as u8);
        rand_indexes.push(index);
    }

    for i in rand_indexes {
        let _ = mem::replace(&mut word[i as usize], '_');
    }

    let word: String = word.iter().collect();
    word
}
