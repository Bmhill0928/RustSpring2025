fn concat_strings(s1: &String, s2: &String) -> String {
    // Your code here
    let mut result = String::from(&s1[..]);
    result.push_str(s2);
    result
}

fn main() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let result = concat_strings(&s1, &s2);
    println!("{}", result); // Should print: "Hello, World!"
}