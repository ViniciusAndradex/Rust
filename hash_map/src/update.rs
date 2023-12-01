use std::collections::HashMap;

pub fn overwriting(map: &mut HashMap<String, i32>, key: String, value: i32) -> &mut HashMap<String, i32> {
    map.insert(key, value);
    map
}