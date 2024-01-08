use std::f32::consts::PI;

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

pub fn test() {
    println!("{0} test {1}, {1} test {0}", "first", "second");
    println!("{number} {string}", string="String", number=1);
    println!("Base 10. {}", 6942);
    println!("Base 2. {:b}", 6942);
    println!("Base 8. {:o}", 6942);
    println!("Base 16. {:x}", 6942);

    println!("number: {number:0>6}", number=10);
    println!("number: {number:0<6}", number=10);
    println!("{number:0>width$}", number=1, width=5);

    println!("My name is {0}, {1} {0}", "Bond", "james");

    println!("Pi is roughly {0:.6}", PI);

    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // Pretty print
    println!("{:#?}", peter);
}