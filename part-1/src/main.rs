use std::thread;

fn main() {
    let thread_join_handle = thread::spawn(move || {
        println!("Hello from thread!")
    });
    println!("Hello from main thread!");
    let _ = thread_join_handle.join();
}
