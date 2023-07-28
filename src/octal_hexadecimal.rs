/*
* Crea una función que reciba un número decimal y lo trasforme a Octal y Hexadecimal.
*/

pub fn to_octal(dec: u32) -> String {
    const OCT_BASE: [char; 8] = ['0', '1', '2', '3', '4', '5', '6', '7'];
    if dec < 1 {
        String::new()
    } else {
        let mut str = to_octal(dec / 8);
        let index = dec % 8;
        str.push(OCT_BASE[index as usize]);
        str
    }
}

pub fn to_hexadecimal(dec: u32) -> String {
    const HEX_BASE: [char; 16] = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
    ];
    if dec < 1 {
        String::new()
    } else {
        let mut str = to_hexadecimal(dec / 16);
        let index = dec % 16;
        str.push(HEX_BASE[index as usize]);
        str
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn to_oct_ok() {
        assert_eq!(to_octal(1000), "1750");
    }

    #[test]
    fn to_oct_ko() {
        assert_ne!(to_octal(1000), "80");
    }

    #[test]
    fn to_hex_ok() {
        assert_eq!(to_hexadecimal(1000), "3E8");
    }

    #[test]
    fn to_hex_ko() {
        assert_ne!(to_hexadecimal(1000), "80");
    }
}
