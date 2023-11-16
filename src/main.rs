use crate::coin::State;
use crate::options::add_one;

mod type_enum;
mod message;
mod options;
mod coin;

fn main() {
    #[warn(unused_variables)]
    let _local = type_enum::IpAddress{version:type_enum::IpVersion::V4,
                                     address:String::from("127.0.0.1")};
    #[warn(unused_variables)]
    let _loopback = type_enum::IpAddress{version:type_enum::IpVersion::V6,
                                         address:String::from("::1")};
    #[warn(unused_variables)]
    let _local_enum = type_enum::EnumIpAddress::V4_1(127, 0, 0, 1);

    #[warn(unused_variables)]
    let _local_str_enum = type_enum::EnumIpAddress::V4(String::from("127.0.0.1"));


    let x = message::Message::Move {x: 45, y: 85};
    x.movement();
    x.exit();

    let x: i8 = 9;
    let t:Option<i8> = Some(5);
    let none_type: Option<i8> = None;

    println!("add {:?} 1 is {:?}", t, add_one(&t));

    if let Some(2) = t {
        println!("Five");
    } else { println!("Error") }

    let coin = coin::Coin::Quarter(State::Alabama);
    println!("Coin in cents is {}", coin.value_in_cents());
}
