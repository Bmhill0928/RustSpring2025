fn clone_and_modify(s: &String) -> String {
    // Your code here
    let mut cloned_word = s.clone();
    cloned_word.push_str("World!");
    cloned_word
}

fn main() {
    let s = String::from("Hello, ");
    let modified = clone_and_modify(&s);
    println!("Original: {}", s); // Should print: "Original: Hello, "
    println!("Modified: {}", modified); // Should print: "Modified: Hello, World!"
}