#[derive(Debug)]
pub enum Message {
    Exit,
    Move{ x: i32, y: i32 },
    Write(String),
    ColorChange(i32, i32, i32)
}

impl Message {
    pub fn exit(&self) {
        println!("Exit!")
    }
}

impl Message {
    pub fn movement(&self) {
        match self {
            Message::Move { x, y} => println!("Move in x: {x} and y: {y} direction"),
            _ => println!("Error, Not Move type")
        }
    }
}