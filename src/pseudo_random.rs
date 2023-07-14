/*
 * Crea un generador de números pseudoaleatorios entre 0 y 100.
 * - No puedes usar ninguna función "random" (o semejante) del lenguaje de programación seleccionado.
 *
 * Es más complicado de lo que parece...
 */

use std::time::Instant;

pub fn psudo_random() -> usize {
    let now = Instant::now();
    println!("{:?}", now.);
    5
}
