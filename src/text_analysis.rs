/*
* Crea un programa que analice texto y obtenga:
* - Número total de palabras.
* - Longitud media de las palabras.
* - Número de oraciones del texto (cada vez que aparecen un punto).
* - Encuentre la palabra más larga.
*
* Todo esto utilizando un único bucle.
*/

use std::io;

pub fn analyze_text() {
    println!("Ingrese el texto a analizar:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read the line");
    input = input.trim().to_string();

    let words_count = count_words_from_text(&input);
    let sentences_count = count_sentences_from_text(&input);
    let mean_length = get_words_len_mean(&input);
    let longest_word = get_longest_word(&input);

    println!("El número total de palabras es: {words_count}");
    println!("La longitud media es: {mean_length}");
    println!("El número total de oraciones es: {sentences_count}");
    println!("La palabra más largas es: {longest_word}");
}

fn split_text_by(text: &str, delimiter: char) -> Vec<&str> {
    text.split(delimiter).collect()
}

fn count_words_from_text(text: &str) -> usize {
    split_text_by(text, ' ').len()
}

fn count_sentences_from_text(text: &str) -> usize {
    split_text_by(text, '.').len()
}

fn get_words_len_mean(text: &str) -> usize {
    let words = split_text_by(text, ' ');
    let mut total_len: usize = 0;
    words.iter().for_each(|x| total_len += x.len());
    total_len / words.len()
}

fn get_longest_word(text: &str) -> &str {
    let words = split_text_by(text, ' ');
    let mut longest_word = "";
    words.iter().for_each(|x| {
        if x.len() > longest_word.len() {
            longest_word = *x
        }
    });
    longest_word
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_words_number_ok() {
        let words = count_words_from_text("Hola, mundo. I am testing");
        assert_eq!(words, 5);
    }

    #[test]
    fn get_words_number_ko() {
        let words = count_words_from_text("Hola, mundo. I am testing");
        assert_ne!(words, 6);
    }

    #[test]
    fn get_sentences_number_ok() {
        let sentences = count_sentences_from_text("Hola, mundo. I am testing");
        assert_eq!(sentences, 2);
    }

    #[test]
    fn get_sentences_number_ko() {
        let sentences = count_sentences_from_text("Hola, mundo. I am testing");
        assert_ne!(sentences, 3);
    }

    #[test]
    fn get_words_mean_ok() {
        let mean = get_words_len_mean("Hola, mundo. I am testing");
        assert_eq!(mean, 4);
    }

    #[test]
    fn get_words_mean_ko() {
        let mean = get_words_len_mean("Hola, mundo. I am testing");
        assert_ne!(mean, 5);
    }

    #[test]
    fn get_longest_word_ok() {
        let word = get_longest_word("Hola, mundo. I am testing");
        assert_eq!(word, "testing");
    }

    #[test]
    fn get_longest_word_ko() {
        let word = get_longest_word("Hola, mundo. I am testing");
        assert_ne!(word, "mundo.");
    }
}
