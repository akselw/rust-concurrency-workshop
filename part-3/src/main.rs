use std::sync::mpsc::Receiver;

fn main() {
    let receiver = producers();
    while let Ok(x) = receiver.recv() {
        println!("Got: {x}")
    }
}

fn producers() -> Receiver<i32> {
    let (sender, receiver) = std::sync::mpsc::channel::<i32>();

    (0..10).for_each(|x| {
        // Clone the sender outside the thread scope
        let sender = sender.clone();
        std::thread::spawn(move || {
            // So that the cloned sender can be moved into the thread, giving it ownership
            sender.send(x).expect("Couldn't send message");
        });
    });

    receiver
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
