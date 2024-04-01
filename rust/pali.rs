use std::io;
fn main() {
    fn is_palindrome(s: &str)-> bool {
        let reversed: String = s.chars().rev().collect();
        s == reversed
    }
    
    let mut inp = String::new();
    io::stdin().read_line(&mut inp).expect("Failed");
    let inp=inp.trim();
    if is_palindrome(inp) {
        println!("{} is a palindrome", inp);
    }
    else {
        println!("{} is not a palindrome", inp);
    }
    
}
