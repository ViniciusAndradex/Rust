use std::collections::HashMap;

pub fn investigator(hash: &mut HashMap<String, i32>, key: String) -> &mut HashMap<String, i32> {
    hash.entry(key).or_insert(1000000);
    hash
}