/*
 * Crea una función que dibuje una escalera según su número de escalones.
 * - Si el número es positivo, será ascendente de izquiera a derecha.
 * - Si el número es negativo, será descendente de izquiera a derecha.
 * - Si el número es cero, se dibujarán dos guiones bajos (__).
 */

const SPACE: &str = "  ";
const STAIR_UP: &str = "_|";
const STAIR_DOWN: &str = "|_";
const FLOOR: &str = "_";

pub fn print_stairs(num: i8) {
    if num == 0 {
        println!("{}", FLOOR.repeat(2));
    } else if num < 0 {
        let num = num.abs() as usize;
        for n in 0..num {
            let spaces = SPACE.repeat(n);
            println!("{spaces}{STAIR_DOWN}");
        }
    } else {
        let num = num.abs() as usize;
        for n in (0..num).rev() {
            let spaces = SPACE.repeat(n);
            println!("{spaces}{STAIR_UP}");
        }
    }
}
