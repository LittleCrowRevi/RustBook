use std::io;
use serde::de::StdError;

pub fn read_num_input(input_phrase: String, error_phrase: String) -> i32 {
    loop {
        println!("{}", input_phrase);
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Couldn't read input!");

        match input.trim().parse() {
            Ok(n) => break n,
            Err(_) => {println!("{}", error_phrase);
            continue;}
        }
    }
}

pub fn read_string_input(input_phrase: String, error_phrase: String) -> String {
    loop {
        println!("{}", input_phrase);
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Couldn't read input!");

        match input.trim().parse() {
            Ok(n) => break n,
            Err(_) => {println!("{}", error_phrase);
                continue;}
        }
    }
}


pub fn read_input(input_phrase: String) -> String
{
        println!("{}", input_phrase);
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Couldn't read input!");

        input
}