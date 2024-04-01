fn main() {
    let ori_str = String::from("Hello, world");
    let slice = &ori_str[0..5];
    println!("Original_string:{}", ori_str);
    println!("Slice:{}", slice);
    
    let mut modified_str = ori_str.clone();
    modified_str.push_str("Good Bye");
    
    println!("Modified_string:{}", modified_str);
    println!("Slice:{}", slice);
    
    let newstr = String::from("This is new");
    let word = get_slice(&newstr, 0, 4);
    println!("Word Slice:{}", word);
    
    fn get_slice(s: &str, start:usize, end:usize)-> &str {
        &s[start..=end]
    }
    
}
