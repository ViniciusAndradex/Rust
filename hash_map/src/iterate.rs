use std::collections::HashMap;

pub fn iterator(map: &HashMap<String, i32>) {
    for (key, value) in  map {
        println!("Key: {key} and Value {value}");
    }
}