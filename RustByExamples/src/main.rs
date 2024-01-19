use std::fs::File;
use std::io::ErrorKind;

mod formated_prints;
mod primitive_types;
mod custom_type;

fn main() {
    println!("Hello, world!");
    println!("I'm a Rustacean!");

    let a = 5;

    formated_prints::test();
    primitive_types::tuples();

    // Recover Errors, panic!

    let greeting_file = File::open("hello_world.txt").unwrap_or_else(0|error| match error.kind() { // Tratamento de erro
        ErrorKind::NotFound => match File::create("hello_world.txt") {
            Ok(file) => file,
            Err(error) => panic!("Problem creating the file: {:?}", error),
        },
        other_error => panic!("Problem opening the file: {:?}", error)
    });
}
