/*
* Crea un generador de números pseudoaleatorios entre 0 y 100.
*/

use std::time::Instant;

pub fn psudo_random() -> usize {
    let now = Instant::now();
    println!("{:?}", now.);
    5
}
