/*
    1. Primeira opção de Chamada Idiomtica
    use std::io::Result as IOResult;
    use std::fmt::Result;
    2. Segunda opção de Chamada Idiomtica
    use std::io
    use std::fmt
*/

/*
    Nested path used list.
    use std::{io::Result, f64, fmt::Alignment};
    use std::io{self, Write} -> Está linha indica que eu quero import do próprio io e do Write.
*/

mod front_of_house;
mod back_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    //Absolute path
    // use crate::front_of_house::hosting::add_to_waitlist();
    //relative path
    // use front_of_house::hosting::add_to_waitlist();
    //Completely path
    hosting::add_to_waitlist();
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
}

fn deliver_order() {}

mod customer {
    use super::front_of_house::hosting;
    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
}