mod password_generator;

fn main() {
    println!("{}", password_generator::password_generator(8, false, false, false));
    println!("{}", password_generator::password_generator(10, true, false, false));
    println!("{}", password_generator::password_generator(12, false, true, false));
    println!("{}", password_generator::password_generator(14, false, false, true));
    println!("{}", password_generator::password_generator(16, true, false, true));
    println!("{}", password_generator::password_generator(9, true, true, false));
    println!("{}", password_generator::password_generator(11, true, true, true));
}
