use std::collections::HashMap;
use crate::iterate::iterator;

mod access_value;
mod iterate;
mod ownership;
mod update;
mod verify_key_in_map;

fn main() {
    let mut scores: HashMap<String, i32> = HashMap::new();

    scores.insert(String::from("Blue"), 55);
    scores.insert(String::from("Yellow"), 10);

    println!("{:?}", scores);

    let key = String::from("Blue");
    let key_2 = String::from("Brown");
    let score = access_value::access(&scores, &key);
    let score_2: i32 = access_value::access(&scores, &key_2);

    println!("The value for key {key} is {score}");
    println!("The value for key {key_2} is {score_2}");

    println!("\nThe values in HashMap is: ");
    iterate::iterator(&scores);

    let str = String::from("I lost my value for ownership");
    let num = 32;

    let new_hash = ownership::create_hash_map(str, num);
    iterate::iterator(&new_hash);

    let _x = update::overwriting(&mut scores, String::from("Blue"), 55);
    let x = update::overwriting(&mut scores, String::from("Yellow"), 0);
    iterate::iterator(&x);
    let mut x = update::overwriting(&mut scores, String::from("Blue"), 88);
    iterate::iterator(&x);

    verify_key_in_map::investigator(&mut x, String::from("Blue"));
    verify_key_in_map::investigator(&mut x, String::from("Black"));
    iterator(&x);


    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
}