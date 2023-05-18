// Escribe un programa que reciba un texto y transforme lenguaje natural a
// "lenguaje hacker" (conocido realmente como "leet" o "1337"). Este lenguaje
// se caracteriza por sustituir caracteres alfanuméricos.

use std::collections::HashMap;

pub fn leet_translator(word: &str) -> String {
    let mut leet_dict: HashMap<char, &str> = HashMap::new();
    leet_dict.insert('a', "4");
    leet_dict.insert('b', "8");
    leet_dict.insert('c', "[");
    leet_dict.insert('d', ")");
    leet_dict.insert('e', "3");
    leet_dict.insert('f', "|=");
    leet_dict.insert('g', "&");
    leet_dict.insert('h', "#");
    leet_dict.insert('i', "1");
    leet_dict.insert('j', ",_|");
    leet_dict.insert('k', ">|");
    leet_dict.insert('l', "1");
    leet_dict.insert('m', "^^");
    leet_dict.insert('n', "^/");
    leet_dict.insert('o', "0");
    leet_dict.insert('p', "|*");
    leet_dict.insert('q', "(_,)");
    leet_dict.insert('r', "12");
    leet_dict.insert('s', "5");
    leet_dict.insert('t', "7");
    leet_dict.insert('u', "(_)");
    leet_dict.insert('v', "|/");
    leet_dict.insert('w', "VV");
    leet_dict.insert('x', "><");
    leet_dict.insert('y', "j");
    leet_dict.insert('z', "2");

    let mut translation = String::new();

    for i in word.to_lowercase().chars() {
        match leet_dict.get(&i) {
            Some(j) => translation.push_str(j),
            None => translation.push(i),
        }
    }

    translation
}
