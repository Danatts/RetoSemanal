/*
 * Crea 3 funciones, cada una encargada de detectar si una cadena de
 * texto es un heterograma, un isograma o un pangrama.
 * - Debes buscar la definición de cada uno de estos términos.
 */

use std::collections::HashMap;

fn remove_whitespace(text: &mut String) {
    text.retain(|c| !c.is_whitespace());
}

pub fn heterogram(text: &str) -> bool {
    let mut new_text = text.to_string().to_lowercase();
    remove_whitespace(&mut new_text);

    let mut collection: HashMap<char, usize> = HashMap::new();

    for ch in new_text.chars() {
        let count = collection.entry(ch).or_insert(0);
        *count += 1;
    }

    for val in collection.values() {
        if val > &1 {
            return false;
        }
    }

    true
}

pub fn isogram(text: &str) -> bool {
    let mut new_text = text.to_string().to_lowercase();
    remove_whitespace(&mut new_text);

    let mut collection: HashMap<char, usize> = HashMap::new();

    for ch in new_text.chars() {
        let count = collection.entry(ch).or_insert(0);
        *count += 1;
    }

    let count = collection
        .get(&new_text.chars().next().unwrap())
        .unwrap()
        .to_owned();

    for val in collection.values() {
        if *val != count {
            return false;
        }
    }

    true
}

pub fn pangram(text: &str) -> bool {
    let letters = String::from("abcdefghijklmnopqrstuvwxyz");

    let mut new_text = text.to_string().to_lowercase();
    remove_whitespace(&mut new_text);

    let mut collection: HashMap<char, usize> = HashMap::new();

    for ch in letters.chars() {
        collection.entry(ch).or_insert(0);
    }

    for ch in new_text.chars() {
        let count = collection.entry(ch).or_insert(0);
        *count += 1;
    }

    for val in collection.values(){
        if *val == 0 {
            return false;
        }
    }

    true
}
