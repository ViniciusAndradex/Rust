fn main() {
    let a: Vec<i32> = vec![1, -22, 3];
    let b: [i32;3] = [1, 2, 3];

    let mut iter_map = b.iter().map(|x| x + 5);
    println!("{:?}", iter_map.next());
    println!("{:?}", b);
    assert_eq!(iter_map.next(), Some(7));

    let mut iter = a.iter().filter(|&x| *x > 0).collect();

    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), Some(&2));
    println!("{:?}", iter);
}
