// String
// The type of string literal "hello, world" is &str, e.g let s: &str = "hello, world".

// Str and &str
// 🌟 We can't use str type in normal ways, but we can use &str.

// Fix error without adding new line
fn one() {
    let s: &str = "hello, world";

    println!("Success!");
}

// 🌟🌟 We can only use str by boxing it, & can be used to convert Box<str> to &str

// Fix the error with at least two solutions
fn two() {
    let s: Box<str> = "hello, world".into();
    greetings(&s)
}

fn greetings(s: &str) {
    println!("{}",s)
}

// String
// String type is defined in std and stored as a vector of bytes (Vec), but guaranteed to always be a
// valid UTF-8 sequence. String is heap allocated, growable and not null terminated.


// Fill the blank
fn three() {
    let mut s = String::new();
    s.push_str("hello, world");
    s.push('!');

    assert_eq!(s, "hello, world!");

    println!("Success!");
}


// Fix all errors without adding newline
fn four() {
    let mut s = String::from("hello");
    s.push(',');
    s.push_str(" world");
    s += "!";

    println!("{}", s);
}

// 🌟🌟 replace can be used to replace substring

// Fill the blank
fn five() {
    let s = String::from("I like dogs");
    // Allocate new memory and store the modified string there
    let s1 = s.replace("dogs", "cats");

    assert_eq!(s1, "I like cats");

    println!("Success!");
}

// More String methods can be found under String module.
// 🌟🌟 You can only concat a String with &str, and String's ownership can be moved to another variable.


// Fix errors without removing any line
fn six() {
    let s1: String = String::from("hello,");
    let s2: String = String::from("world!");
    let s3 = s1 + &s2;
    assert_eq!(s3, "hello,world!");
    println!("{}", s3);
}

// &str and String
//
// Opposite to the seldom using of str, &str and String are used everywhere!
//
// 🌟🌟 &str can be converted to String in two ways

// Fix error with at least two solutions
fn seven() {
    let s: &str = "hello, world";
    greetings_1(s.to_string()) // String::from(s)
}

fn greetings_1(s: String) {
    println!("{}", s)
}

// 🌟🌟 We can use String::from or to_string to convert a &str to String

// Use two approaches to fix the error and without adding a new line
fn eight() {
    let s: String = "hello, world".to_string();
    let s1: &str = &s;

    println!("Success!");
}

// String index
// 🌟🌟🌟 You can't use index to access a char in a string, but you can use slice &s1[start..end].

fn eleven() {
    let s1 = String::from("hi,中国");
    let h = &s1[0..1]; // Modify this line to fix the error, tips: `h` only takes 1 byte in UTF8 format
    assert_eq!(h, "h");

    let h1 = &s1[3..6]; // Modify this line to fix the error, tips: `中`  takes 3 bytes in UTF8 format
    assert_eq!(h1, "中");

    println!("Success!");
}


fn twelve() {
    // Fill the blank to print each char in "你好，世界"
    for c in "你好，世界".chars() {
        println!("{}", c)
    }
}

