pub enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(State),
}

#[derive(Debug)]
pub enum State {
    Alaska,
    Alabama,
    Oklahoma,
    Detroit
}

impl Coin {
    pub fn value_in_cents(&self) -> u32 {
        match self {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("The quarter is the {:?} state.", state);
                25
            }
        }
    }
}