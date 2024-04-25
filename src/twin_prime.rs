/*
* Crea un programa que encuentre y muestre todos los pares de números primos
* gemelos en un rango concreto.
* El programa recibirá el rango máximo como número entero positivo.
 */

pub fn is_prime(num: usize) -> bool {
    let num = num as f32;
    if num % 2.0 == 0.0 || num % 3.0 == 0.0 || num % 5.0 == 0.0 {
        return false;
    } else if num.sqrt().fract() == 0.0 {
        return false;
    } else {
        true
    }
}
