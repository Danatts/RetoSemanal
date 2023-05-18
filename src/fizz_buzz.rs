// Escribe un programa que muestre por consola (con un print) los
// números de 1 a 100 (ambos incluidos y con un salto de línea entre
// cada impresión), sustituyendo los siguientes:
// - Múltiplos de 3 por la palabra "fizz".
// - Múltiplos de 5 por la palabra "buzz".
// - Múltiplos de 3 y de 5 a la vez por la palabra "fizzbuzz".

pub fn print_fizz_buzz() -> () {
    for num in 1..=100 {
        match num {
            num if num % 3 == 0 && num % 5 == 0 => println!("fizzbuzz"),
            num if num % 3 == 0 => println!("fizz"),
            num if num % 5 == 0 => println!("buzz"),
            _ => println!("{num}"),
        }
    }
}
