use std::io;

use crate::utilities::input_util;


pub fn convert() {

    println!("What would you like to convert? F or C? ");

    let type_convert = input_util::read_string_input(
        String::from("Type C or F!"),
        String::from("Please type a letter!"));

        if type_convert.to_uppercase() == "F" {
            println!("Converting to Fahrenheit!");
            convert_to_celsius();
        }
        else if type_convert == "C" {

        }
    }


fn convert_to_celsius() {

    let temperature = input_util::read_num_input(
        String::from("Please input the temp you'd like to convert!"),
        String::from("Type a number!"));

    println!("{}°C", (temperature - 32) * 5/9)
}