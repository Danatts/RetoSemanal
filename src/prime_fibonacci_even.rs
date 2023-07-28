/*
* Escribe un programa que, dado un número, compruebe y muestre si es primo, fibonacci y par.
*/

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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn is_even_ok() {
        assert!(is_even(4));
    }

    #[test]
    fn is_even_ko() {
        assert!(!is_even(5));
    }

    #[test]
    fn is_prime_ok() {
        assert!(is_prime(89));
    }

    #[test]
    fn is_prime_ko() {
        assert!(!is_prime(54));
    }

    #[test]
    fn is_fibonacci_ok() {
        assert!(is_fibonacci(55));
    }

    #[test]
    fn is_fibonacci_ko() {
        assert!(!is_fibonacci(10));
    }
}
