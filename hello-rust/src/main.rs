use ferris_says::say;
use std::io::{stdout, BufWriter};

fn main() {
   let stdout = stdout();
    let message = String::from("Hello man");
    let width = message.chars().count();

    let mut writter = BufWriter::new(stdout.lock());
    say(&message, width, &mut writter).unwrap();
}