use std::collections::HashMap;

pub fn create_hash_map(key: String, value: i32) -> HashMap<String, i32> {
    let mut hash: HashMap<String, i32> = HashMap::new();

    hash.insert(key, value);
    hash
}