// Todas as Atividades do capítulo de variáveis serão copiados para está área.

//A variable can be used only if it has been initialized.
// Fix the error below with least amount of modification to the code
fn one() {
    let x: i32 = 5; // Uninitialized but used, ERROR !
    let _y: i32; // Uninitialized but also unused, only a Warning !

    assert_eq!(x, 5);
    println!("Success!");
}

// Use mut to mark a variable as mutable.
// Fill the blanks in the code to make it compile
fn two() {
    let mut x = 1;
    x += 2;

    assert_eq!(x, 3);
    println!("Success!");
}

// A scope is the range within the program for which the item is valid.
// Fix the error below with least amount of modification
fn three() {
    let x: i32 = 10;
    let y: i32 = 11;
    {
        println!("The value of x is {} and value of y is {}", x, y);
    }
    println!("The value of x is {} and value of y is {}", x, y);
}


// Fix the error with the use of define_x
fn four() {
    define_x();
}

fn define_x() {
    let x: &str = "hello";

    println!("{}, world", x);

}

// You can declare a new variable with the same name as a previous variable, here we can say the first one is shadowed by the second one.

// Only modify `assert_eq!` to make the `println!` work(print `42` in terminal)
fn five() {
    let x: i32 = 5;
    {
        let x: i32 = 12;
        assert_eq!(x, 12);
    }

    assert_eq!(x, 5);

    let x: i32 = 42;
    println!("{}", x); // Prints "42".
}


// Remove a line in the code to make it compile
fn six() {
    let mut x: i32 = 1;
    x = 7;
    // Shadowing and re-binding
    // let x = x;
    x += 3;


    let y = 4;
    // Shadowing
    let y = "I can also be bound to text!";

    println!("Success!");
}

// Fix the warning below with :
#[warn(unused_variables)]
fn seven() {
    // Solution 1: _x
    let x = 1;
}

// Warning: unused variable: `x`

// We can use a pattern with let to destructure a tuple to separate variables.

// Fix the error below with least amount of modification
fn eight() {
    let (mut x, y) = (1, 2);
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);

    println!("Success!");
}

// Introduced in Rust 1.59: You can now use tuple, slice, and struct patterns as the left-hand side of an assignment.

fn nine() {
    let (x, y);
    (x,..) = (3, 4);
    [.., y] = [1, 2];
    // Fill the blank to make the code work
    assert_eq!([x,y], [3, 2]);

    println!("Success!");
}