mod text_type;

fn main() {
    println!("{}", text_type::heterogram("abcdefg"));
    println!("{}", text_type::heterogram("aabcdefg"));
    println!("{}", text_type::isogram("abcdefg"));
    println!("{}", text_type::isogram("aabbcc"));
    println!("{}", text_type::isogram("aaabbcc"));
    println!("{}", text_type::pangram("abcdefghijklmnopqrstuvwxyz"));
    println!("{}", text_type::pangram("aaabbcc"));
}
