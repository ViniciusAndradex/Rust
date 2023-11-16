enum Options<T> {
    Some(T),
    None
}


pub fn add_one(&x: &Option<i8>) -> Option<i8> {
    match x {
        None => None,
        Some(i) => Some(i + 1)
    }
}
