// 🌟🌟 Methods are similar to functions: Declare with fn, have parameters and a return value. Unlike
// functions, methods are defined within the context of a struct (or an enum or a trait object), and
// their first parameter is always self, which represents the instance of the struct the method is being called on.

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Complete the area method which return the area of a Rectangle.
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn one() {
    let rect1 = Rectangle { width: 30, height: 50 };

    assert_eq!(rect1.area(), 1500);

    println!("Success!");
}


// 🌟🌟 self will take the ownership of current struct instance, however, &self will only borrow a reference from the instance.
// Only fill in the blanks, DON'T remove any line!
#[derive(Debug)]
struct TrafficLight {
    color: String,
}

impl TrafficLight {
    pub fn show_state(&self)  {
        println!("the current state is {}", self.color);
    }
}
fn two() {
    let light = TrafficLight{
        color: "red".to_owned(),
    };
    // Don't take the ownership of `light` here.
    light.show_state();
    // ... Otherwise, there will be an error below
    println!("{:?}", light);
}


// 🌟🌟 The &self is actually short for self: &Self. Within an impl block, the type Self is an alias
// for the type that the impl block is for. Methods must have a parameter named self of type Self for
// their first parameter, so Rust lets you abbreviate this with only the name self in the first parameter spot.

struct TrafficLight_3 {
    color: String,
}

impl TrafficLight_3 {
    // Using `Self` to fill in the blank.
    pub fn show_state(&self)  {
        println!("the current state is {}", self.color);
    }

    // Fill in the blank, DON'T use any variants of `Self`.
    pub fn change_state(&mut self) {
        self.color = "green".to_string()
    }
}
fn three() {
    println!("Success!");
}


// 🌟🌟 All functions defined within an impl block are called associated functions because they’re
// associated with the type named after the impl. We can define associated functions that don’t have
// self as their first parameter (and thus are not methods) because they don’t need an instance of the type to work with.

#[derive(Debug)]
struct TrafficLight_4 {
    color: String,
}

impl TrafficLight_4 {
    // 1. Implement an associated function `new`,
    // 2. It will return a TrafficLight contains color "red"
    // 3. Must use `Self`, DONT use `TrafficLight` in fn signatures or body
    pub fn new() -> TrafficLight {
        TrafficLight { color: "red".to_string() }
    }

    pub fn get_state(&self) -> &str {
        &self.color
    }
}

fn four() {
    let light = TrafficLight::new();
    assert_eq!(light.get_state(), "red");

    println!("Success!");
}


// 🌟 Each struct is allowed to have multiple impl blocks.


struct Rectangle {
    width: u32,
    height: u32,
}

// Using multiple `impl` blocks to rewrite the code below.
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    fn show_result(&self) -> u32 {
        self.area()
    }
}


fn five() {
    println!("Success!");
}


// 🌟🌟🌟 We can also implement methods for enums.


#[derive(Debug)]
enum TrafficLightColor {
    Red,
    Yellow,
    Green,
}

// Implement TrafficLightColor with a method.
impl TrafficLightColor {
    fn color(&self) -> String {
        match self {
            TrafficLightColor::Red => String::from("red"), // Self::Red would also work.
            TrafficLightColor::Yellow => String::from("yellow"),
            TrafficLightColor::Green => String::from("green")
        }
    }
}

fn six() {
    let c = TrafficLightColor::Yellow;

    assert_eq!(c.color(), "yellow");

    println!("{:?}",c);
}