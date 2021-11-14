use std::io;

use crate::utilities::input_util;


pub fn convert() {

    println!("What would you like to convert? F or C? ");

    let mut type_convert = input_util::read_string_input(
        "Type F or C! ".parse().unwrap(),
        "Please input a temp type!".parse().unwrap());

        if type_convert.to_uppercase() == "F" {
            println!("Converting to Fahrenheit!");
            convert_to_celsius();
        }
        else if type_convert == "C" {

        }
    }


fn convert_to_celsius() {

    let mut temperature = input_util::read_num_input(
        "Please input the temp you'd like to convert!".parse().unwrap(),
        "Input a number!".parse().unwrap());

    println!("{}", (temperature - 32) * 5/9)
}