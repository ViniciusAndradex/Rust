use std::f32::consts::PI;

// impl Display for City {
//     // `f` is a buffer, and this method must write the formatted string into it.
//     fn fmt(&self, f: &mut Formatter) -> fmt::Result {
//         let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
//         let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };
//
//         // `write!` is like `format!`, but it will write the formatted string
//         // into a buffer (the first argument).
//         write!(f, "{}: {:.3}°{} {:.3}°{}",
//                self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
//     }
// }

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