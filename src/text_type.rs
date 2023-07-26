// Crea 3 funciones, cada una encargada de detectar si una cadena de
// texto es un heterograma, un isograma o un pangrama.
// - Debes buscar la definición de cada uno de estos términos.

use std::{collections::HashMap, usize};

const LETTERS: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

fn sentence_map(s: &str) -> HashMap<char, usize> {
    let mut normalized_s = s.to_lowercase().to_string();
    normalized_s.retain(|c| !c.is_whitespace());

    let mut map = HashMap::new();
    for ch in normalized_s.chars() {
        let count = map.entry(ch).or_insert(0);
        *count += 1
    }

    map
}

// Heterogram: Each word in a sentence only appears once
fn is_heterogram(s: &str) -> bool {
    let dic = sentence_map(s);

    for val in dic.values() {
        if val > &1 {
            return false;
        }
    }

    true
}

// Isogram: Each word in a sentence appears the same number of times
fn is_isogram(s: &str) -> bool {
    let dic = sentence_map(s);

    let mut count = 0;

    for val in dic.values() {
        if count == 0 {
            count = *val
        }
        if *val != count {
            return false;
        }
    }

    true
}

// Pangram: All letters of the alphabet appers at least once in a sentence
fn is_pangram(s: &str) -> bool {
    let mut alphabet_map: HashMap<char, usize> = HashMap::new();

    for ch in LETTERS {
        alphabet_map.entry(ch).or_insert(0);
    }

    let dic = sentence_map(s);

    for ch in dic.keys() {
        let count = alphabet_map.entry(*ch).or_insert(0);
        *count += 1;
    }

    for val in alphabet_map.values() {
        if *val == 0 {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod test {
    use super::{is_heterogram, is_isogram, is_pangram};

    #[test]
    fn test_heterograma_ok() {
        let heterograma = "Hiperblanduzcos";

        assert_eq!(is_heterogram(heterograma), true);
    }

    #[test]
    fn test_heterograma_ko() {
        let heterograma = "Hiperblanduzcas";

        assert_eq!(is_heterogram(heterograma), false);
    }

    #[test]
    fn test_isograma_ok() {
        let isograma = "HiperblanduzcosHiperblanduzcos";

        assert_eq!(is_isogram(isograma), true);
    }

    #[test]
    fn test_isograma_ko() {
        let isograma = "HiperblanduzcosHiperblanduzcas";

        assert_eq!(is_isogram(isograma), false);
    }

    #[test]
    fn test_panagrama_ok() {
        let pangrama = "El cadáver de Wamba, rey godo de España, fue exhumado y trasladado en una caja de zinc que pesó un kilo.";

        assert_eq!(is_pangram(pangrama), true);
    }

    #[test]
    fn test_panagrama_ko() {
        let pangrama = "Los pangramas perfectos son los pangramas que son también heterogramas, es decir en los que no se repite ninguna de las letras.";

        assert_eq!(is_pangram(pangrama), false);
    }
}
