fn main() {
    // a closure will automatically borrow a reference to values in the enclosing scope
    let add = |x, y| x + y;

    println!("result: {}", add(1, 2));
    println!("result: {}", add(1, 2));

    // let s = "s"; // shared references like &str implement the Copy trait.
    let s = String::from("s"); // String does NOT implement Copy

    // now `s` is moved to the scope of the closure
    let f = move || {
        println!("{}", s);
    };

    f(); // prints "s"

    // println!("{}", s) // so this errors
}
