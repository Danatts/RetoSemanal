pub fn fizz_buzz() -> () {
    for num in 1..=100 {
        match num {
            num if num % 3 == 0 && num % 5 == 0 => println!("fizzbuzz"),
            num if num % 3 == 0 => println!("fizz"),
            num if num % 5 == 0 => println!("buzz"),
            _ => println!("{num}"),
        }
    }
}
