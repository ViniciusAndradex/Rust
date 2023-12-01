use std::collections::HashMap;

pub fn access(map: &HashMap<String, i32>, key: &String) -> i32 {
    let value: i32 = map.get(key).copied().unwrap_or(0);
    value
}