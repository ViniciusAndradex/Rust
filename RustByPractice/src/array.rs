// Array
// The type of array is [T; Length], as you can see, array's length is part of their type signature.
// So their length must be known at compile time.

// For example, you cant initialize an array like below:
fn init_arr(n: i32) {
    let arr = [1; n];
}
// This will cause an error, because the compiler has no idea of the exact size of the array at compile time.


fn one() {
    // Fill the blank with proper array type
    let arr: [i32; 5] = [1, 2, 3, 4, 5];

    // Modify the code below to make it work
    assert!(arr.len() == 5);

    println!("Success!");
}


fn two() {
    // We can ignore parts of the array type or even the whole type, let the compiler infer it for us
    let arr0: [i32; 3] = [1, 2, 3];
    let arr: [_; 3] = ['a', 'b', 'c'];

    // Fill the blank
    // Arrays are stack allocated, `std::mem::size_of_val` returns the bytes which an array occupies
    // A char takes 4 bytes in Rust: Unicode char
    assert!(std::mem::size_of_val(&arr) == 12);

    println!("Success!");
}


// 🌟 All elements in an array can be initialized to the same value at once.

fn three() {
    // Fill the blank
    let list: [i32; 100] = [1;100] ;

    assert!(list[0] == 1);
    assert!(list.len() == 100);

    println!("Success!");
}


// 🌟 All elements in an array must be of the same type

fn four() {
    // Fix the error
    let _arr = [1, 2, 3];

    println!("Success!");
}


// 🌟 Indexing starts at 0.

fn five() {
    let arr = ['a', 'b', 'c'];

    let ele = arr[0]; // Only modify this line to make the code work!

    assert!(ele == 'a');

    println!("Success!");
}


// 🌟 Out of bounds indexing causes panic.


// Fix the error
fn six() {
    let names: [String;2] = [String::from("Sunfei"), "Sunface".to_string()];

    // `Get` returns an Option<T>, it's safe to use
    let name0 = names.get(0).unwrap();

    // But indexing is not safe
    let _name1 = &names[1];

    println!("Success!");
}
