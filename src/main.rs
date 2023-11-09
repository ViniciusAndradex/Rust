use std::io;

fn index() {
    let a = [4,5,6,7];

    let mut index = String::new();

    io::stdin().read_line(&mut index).expect("Failed to read line");
    let index: usize = index.trim().parse().expect("Index entered is not a number");

    let element = a[index];

    println!("The index is {index} and number of the position is {element}");
}

fn shadow() {
    let spaces = "    ";
    println!("Spaces: {spaces}");

    let spaces = spaces.len();
    println!("{spaces}")
}

fn parse() {
    let guess: u32 = "42".parse().expect("Not a number");
    println!("guess {guess}")
}

fn tuple() {
    let tup: (i8, i8, u32) = (1,2, 999999);
    println!("tup 0 {}", tup.0);
}

fn array() {
    let a: [i32; 3] = [55, 66, 98];
    let b = [3; 5];
    println!("a[2] {}", a[2]);
    println!("b[2] {}", b[2]);
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
    parse();
    tuple();
    array();
    index();
    print_labeled_measurement(32, 'g');
    expression();
    let mut a = six();
    println!("six() {a}");
    a = plus_one(a);
    println!("plus_one in six() {a}");
    let condition = true;
    let ternary = if condition { 5 } else { 6 };
    println!("plus_one in six() {ternary}");

    let mut count = 0;

    let result = loop {
        count += 1;

        if count == 10 {
            let x = count as f32;
            break  x * 1.5;
        }
    };
    println!("This return of loop is {result}");

    'teste_loop: loop {
        println!("Entrei no mais externo");
        'teste: loop {
            println!("Entrei no interno um");
            break 'teste;
        }
        'teste_2: loop {
            println!("Entrei no 2 e vou encerrar o externo.");
            break 'teste_loop;
        }
    }
    println!("Saí dos loops");
    let mut x = six();
    while  x < 19 {
        x += 2;
        println!("X on Loop number is {x}");
    }
    let mut x = 2;
    for  x in 3..9 {
        println!("x: {x}");
    }

}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}")
}

fn statement() {
    let x = 5;
}

fn expression() {
    let x =  {
        let h = 6 + 8;
        h * 9
    };
    println!("X is {:?}", x);
}

fn six() -> i32 {
    6
}

fn plus_one(x: i32) -> i32 {
    x + 1 //expression
    // x+ 1; statement
}