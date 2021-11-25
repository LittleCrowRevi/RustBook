use std::fmt::Display;
use crate::utilities::input_util::read_input;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

pub fn ann() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let ann = loop {
        match read_input(String::from("Inout an announcement!")).trim().parse::<i32>() {
            Ok(a) => break a,
            Err(e) => dbg!("PanicPanic!{}", e)
        };
    };

    let result = longest_with_an_announcement(
        string1.as_str(),
        string2,
        ann
    );
    println!("The longest string is {}", result);
}