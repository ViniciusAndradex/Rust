// 🌟🌟 Use | to match several values, use ..= to match an inclusive range.

fn one() { match_number(11)}
fn match_number(n: i32) {
    match n {
        // Match a single value
        1 => println!("One!"),
        // Fill in the blank with `|`, DON'T use `..` or `..=`
        2 | 5 => println!("match 2 -> 5"),
        // Match an inclusive range
        6..=10 => {
            println!("match 6 -> 10")
        },
        _ => {
            println!("match -infinite -> 0 or 11 -> +infinite")
        }
    }
}


// 🌟🌟🌟 The @ operator lets us create a variable that holds a value, at the same time we are testing that value to see whether it matches a pattern.

struct Point {
    x: i32,
    y: i32,
}

fn two() {
    // Fill in the blank to let p match the second arm
    let p = Point { x: 4, y: 20 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        // Second arm
        Point { x: 0..=5, y: y@ (10 | 20 | 30) } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
}


// Fix the errors
enum Message {
    Hello { id: i32 },
}

fn three() {
    let msg = Message::Hello { id: 11 };

    match msg {
        Message::Hello {
            id: id@ 3..=7,
        } => println!("Found an id in range [3, 7]: {}", id),
        Message::Hello { id: newid@ (10 | 11 | 12) } => {
            println!("Found an id in another range [10, 12]: {}", newid)
        }
        Message::Hello { id } => println!("Found some other id: {}", id),
    }
}


// 🌟🌟 A match guard is an additional if condition specified after the pattern in a match arm that
// must also match, along with the pattern matching, for that arm to be chosen.


// Fill in the blank to make the code work, `split` MUST be used
fn four() {
    let num = Some(4);
    let split = 5;
    match num {
        Some(x) if x < split => assert!(x < split),
        Some(x) => assert!(x >= split),
        None => (),
    }

    println!("Success!");
}


// 🌟🌟 Ignoring remaining parts of the value with ..


// Fill the blank to make the code work
fn five() {
    let numbers = (2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048);

    match numbers {
        (first, .., last) => {
            assert_eq!(first, 2);
            assert_eq!(last, 2048);
        }
    }

    println!("Success!");
}


// 🌟🌟 Using pattern &mut V to match a mutable reference requires you to be very careful, due to V being a value after matching.


// FIX the error with least changing
// DON'T remove any code line
fn six() {
    let mut v = String::from("hello,");
    let r = &mut v;

    match r {
        value => value.push_str(" world!")
    }

    println!("{:?}", v);
}