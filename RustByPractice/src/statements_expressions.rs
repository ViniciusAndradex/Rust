fn example() {
    let x = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;
        // EXPRESSION
        // This expression will be assigned to `y`
        x_cube + x_squared + x
    };

    let z = {
        // STATEMENT
        // The semicolon suppresses this expression and `()` is assigned to `z`
        2 * x;
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}

// Make it work with two ways
fn one() {
    let v = {
        let mut x = 1;
        x += 2; // x + 2
        x
    };

    assert_eq!(v, 3);

    println!("Success!");
}


fn two() {
    let v = {
        let x = 3;
        x
    };

    assert!(v == 3);

    println!("Success!");
}


fn three() {
    let s = sum(1 , 2);
    assert_eq!(s, 3);

    println!("Success!");
}

fn sum(x: i32, y: i32) -> i32 {
    x + y
}