use std::thread;
use std::sync::mpsc::*;

fn main() {
    let receiver = producers();
    while let Ok(x) = receiver.recv() {
        println!("Got: {x}")
    }
}

fn producers() -> Receiver<i32> {
    let (tx, rx) = channel::<i32>();
    for i in 0..10 {
        let tx = tx.clone();
        thread::spawn(move || {
            tx.send(i).unwrap();
        });
    }
    rx
}

#[test]
fn sends_messages_correctly() {
    use std::collections::HashSet;
    // There's no real good way to check that you've actually spawned 10 threads
    // other than to check a number of join handles, but we're just gonna
    // check that the values are produced correctly.

    let receiver = producers();

    let mut results = HashSet::new();
    while let Ok(x) = receiver.recv() {
        results.insert(x);
    }

    assert_eq!(results, HashSet::from_iter(0..10))
}
