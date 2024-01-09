mod formated_prints;
mod primitive_types;
mod custom_type;

fn main() {
    println!("Hello, world!");
    println!("I'm a Rustacean!");

    let a = 5;

    formated_prints::test();
    primitive_types::tuples();
}
