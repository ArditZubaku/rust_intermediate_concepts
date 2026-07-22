use std::thread;

fn main() {
    let handle = thread::spawn(move || {
        // do stuff in a child thread
        println!("{:?}", thread::current().id())
    });

    // do stuff in the main thread
    println!("{:?}", thread::current().id());

    // wait until child thread has exited
    handle.join().unwrap();
}
