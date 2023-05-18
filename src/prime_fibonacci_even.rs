// Escribe un programa que, dado un número, compruebe y muestre si es primo, fibonacci y par.
// Ejemplos:
// - Con el número 2, nos dirá: "2 es primo, fibonacci y es par"
// - Con el número 7, nos dirá: "7 es primo, no es fibonacci y es impar"

// main.rs
// mod prime_fibonacci_even;
//
// fn main() {
//     prime_fibonacci_even::check_all(89);
// }

fn is_even(num: u32) -> bool {
    num % 2 == 0
}

fn is_fibonacci(num: u32) -> bool {
    let mut a = 0;
    let mut b = 1;
    let mut c = a + b;
    loop {
        if c == num {
            break true;
        }
        if c > num {
            break false;
        }
        a = b;
        b = c;
        c = a + b;
    }
}

fn is_prime(num: u32) -> bool {
    let mut current = 2;
    loop {
        if current == num {
            break true;
        }
        if num % current == 0 {
            break false;
        }
        current += 1;
    }
}

pub fn check_all(num: u32) {
    println!("Number: {num}");
    println!("Is it even?: {}", is_even(num));
    println!("Is it a fibonacci number?: {}", is_fibonacci(num));
    println!("Is it prime?: {}", is_prime(num));
}
