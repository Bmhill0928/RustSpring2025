use std::io::{self, Read, Write};

use std::fs::File;

struct Person {
    name: String,
    age: u32,
}

struct Config {
    ourname: String,
    student_id: String,
    port: u16,
}

impl Config {
    fn from_file(path: &str) -> Config {
        let mut file = File::open(path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let mut lines = contents.lines();
        let ourname = lines.next().unwrap().to_string();
        let student_id = lines.next().unwrap().to_string();
        let port = lines.next().unwrap().parse().unwrap();

        Config { ourname, student_id, port }
    }
}

//Reading from console function
fn reading_from_console() {
    let mut buffer = String::new();

    print!("What's your name? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let name = buffer.trim().to_string();
    buffer.clear();

    print!("How old are you? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let age = buffer.trim().parse().unwrap();

    let person = Person { name, age };
    println!("Hi {}, you are {} years old!", person.name, person.age);
}

//Reading from our config function
fn reading_from_file() {
    let config = Config::from_file("config.txt");
    println!("Student Name: {}", config.ourname);
    println!("Student ID: {}", config.student_id);
    println!("port: {}", config.port);
}

//Create our main function to read from console and read from our file
fn main(){
    reading_from_console();
    reading_from_file();
}