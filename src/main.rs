fn shadow() {
    let spaces = "    ";
    println!("Spaces: {spaces}");

    let spaces = spaces.len();
    println!("{spaces}")
}

fn main() {
    let x = 6;
    println!("The value of x is {x}");

    let mut y = 5;
    println!("The value of y is {y}");
    y = 8;
    println!("The value of x is {y}");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60  * 3;
    println!("Hours in seconds {THREE_HOURS_IN_SECONDS}");


    let x = x + 6;

    {
        let x = x * 2;
        println!("Value inner scope {x}");
    }

    println!("Value out scope {x}");

    shadow();

}