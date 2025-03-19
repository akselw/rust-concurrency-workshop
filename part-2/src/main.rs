use std::sync::mpsc::*;

fn main() {
    let receiver = across_the_border();

    while let Ok(x) = receiver.recv() {
        println!("Got: {x}");
    }
}

fn across_the_border() -> Receiver<i32> {
    let (tx, rx) = channel::<i32>();
    for i in 0..10 {
        tx.send(i).unwrap();
    }
    rx
}

#[test]
fn sends_data_correctly() {
    let receiver = across_the_border();
    let mut result = Vec::new();
    while let Ok(x) = receiver.recv() {
        result.push(x);
    }

    assert_eq!(result, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9])
}
