// Escribe un programa que sea capaz de generar contraseñas de forma aleatoria.
// Podrás configurar generar contraseñas con los siguientes parámetros:
// - Longitud: Entre 8 y 16.
// - Con o sin letras mayúsculas.
// - Con o sin números.
// - Con o sin símbolos.
// (Pudiendo combinar todos estos parámetros entre ellos)

// mod password_generator;
// 
// fn main() {
//     println!("{}", password_generator::password_generator(8, false, false, false));
//     println!("{}", password_generator::password_generator(10, true, false, false));
//     println!("{}", password_generator::password_generator(12, false, true, false));
//     println!("{}", password_generator::password_generator(14, false, false, true));
//     println!("{}", password_generator::password_generator(16, true, false, true));
//     println!("{}", password_generator::password_generator(9, true, true, false));
//     println!("{}", password_generator::password_generator(11, true, true, true));
// }

use rand::{thread_rng, Rng};

pub fn password_generator(len: u8, cap: bool, num: bool, symb: bool) -> String {
    let mut valid_chars: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect::<Vec<char>>();
    if cap {
        valid_chars.append(&mut "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect::<Vec<char>>());
    }
    if num {
        valid_chars.append(&mut "1234567890".chars().collect::<Vec<char>>());
    }
    if symb {
        valid_chars.append(&mut "!\"#$%&'/()*+,-.=?¡".chars().collect::<Vec<char>>());
    }

    let mut rng = thread_rng();
    let mut password: String = String::new();
    let mut count: u8 = 0;
    loop {
        let ran_index: usize = rng.gen_range(0..valid_chars.len());
        password.push(valid_chars[ran_index]);
        count += 1;
        if count == len {
            break;
        }
    }
    password
}
