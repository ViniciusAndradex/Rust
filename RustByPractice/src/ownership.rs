fn one() {
    // Use as many approaches as you can to make it work
    let x = String::from("Hello world");
    let y = x.clone();
    let z = &x;
    println!("{}, {}, {}",x, y, z);
}

// Don't modify code in main!
fn two() {
    let s1 = String::from("Hello world");
    let s2 = take_ownership(s1);

    println!("{}", s2);
}

// Only modify the code below!
fn take_ownership(s: String) -> String{
    println!("{}", s);
    s
}
fn three() {
    let s = give_ownership();
    println!("{}", s);
}

// Only modify the code below!
fn give_ownership() -> String {
    let s = String::from("Hello world");
    // Convert String to Vec
    let _s = s.clone().into_bytes(); // as_bytes
    s
}

// Fix the error without removing any code
fn four() {
    let s = String::from("Hello World");

    print_str(&s);

    println!("{}", s);
}

fn print_str(s: &String)  {
    println!("{}",s)
}

// Don't use clone ,use copy instead
fn five() {
    let x = (1, 2, (), "hello");
    let y = x;
    println!("{:?}, {:?}", x, y);
}


// make the necessary variable mutable
fn six() {
    let s = String::from("Hello ");

    let mut s1 = s;

    s1.push_str("World!");

    println!("Success!");
}


fn seven() {
    let x = Box::new(5);

    let mut y = x.clone();      // update this line, don't change other lines!

    *y = 4;

    assert_eq!(*x, 5);

    println!("Success!");
}

// Partial move
// Within the destructuring of a single variable, both by-move and by-reference pattern bindings can be
// used at the same time. Doing this will result in a partial move of the variable, which means that
// parts of the variable will be moved while other parts stay. In such a case, the parent variable cannot
// be used afterwards as a whole, however the parts that are only referenced (and not moved) can still be used.


fn example() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: Box<u8>,
    }

    let person = Person {
        name: String::from("Alice"),
        age: Box::new(20),
    };

    // `name` is moved out of person, but `age` is referenced
    let Person { name, ref age } = person;

    println!("The person's age is {}", age);

    println!("The person's name is {}", name);

    // Error! borrow of partially moved value: `person` partial move occurs
    //println!("The person struct is {:?}", person);

    // `person` cannot be used but `person.age` can be used as it is not moved
    println!("The person's age from person struct is {}", person.age);
}


fn eight() {
    let t = (String::from("hello"), String::from("world"));

    let _s =  t.0;

    // Modify this line only, don't use `_s`
    println!("{:?}", t.1);
}


fn nine() {
    let t = (String::from("hello"), String::from("world"));

    // Fill the blanks
    let (s1, s2) = (&t.0, &t.1);

    println!("{:?}, {:?}, {:?}", s1, s2, t); // -> "hello", "world", ("hello", "world")
}