fn main() {
    let v = vec![1, 2, 3];

    // NOTE: Iterators are typically faster than for loops
    v.clone().into_iter().for_each(|num| println!("{}", num));

    let total: i32 = v
        .clone()
        .into_iter() // 1, 2, 3
        .map(|x| x * 3) // 3, 6, 9
        .filter(|x| *x % 2 == 0) // 6
        // .for_each(|n| println!("{}", n))
        // .sum();
        .sum::<i32>(); // end it with an iterator consumer (like in Java with a stream consumer)
    println!("Total {total}");

    let v2 = v
        .clone()
        .into_iter()
        .map(|x| x * 3)
        .filter(|y| y.is_positive())
        .collect::<Vec<_>>(); // vec![3,6,9]
    println!("v2 {:?}", v2);

    let mut v3 = v2.clone();

    for _ in v {}
    for _ in &v2 {}
    for _ in &mut v3 {}
}
