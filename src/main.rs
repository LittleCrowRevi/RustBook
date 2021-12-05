

mod guessing_game;
mod convert_temp;
mod utilities;
mod area_struct;
mod enums;
mod miu_reqwest;
mod notes;
mod minigrep;

#[tokio::main]
async fn main() {

    /* match miu_reqwest::reqwest_01::reqwest_miu(String::from("https://api.manabot.fun/quotes/1")).await {
      Ok(o) => println!("{}", o),
        Err(e) => println!("{:?}", e)
    };

    println!("Hello, world!");
    // guessing_game::guessing_game::guessing()

    for number in (1..4).rev() {
        println!("{}", number);
    }
    println!("LIFTOFF!!!");

    // convert_temp::convert_temp::convert();

    //area_struct::structs_area::structs();

    // enums::ip_enum::enums()

    // hashmaps counting words in a string
    use std::collections::HashMap;

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);

    notes::traits::traits(); */

    // notes::traits_lifetimes::ann();

    minigrep::minigrep::minigrep()
}
