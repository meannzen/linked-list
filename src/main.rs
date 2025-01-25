use std::thread::{self};

fn main() {
    let t1 = thread::spawn(|| {
        for i in 0..10 {
            println!("t1: {}", i);
        }
    });

    let t2 = thread::spawn(|| {
        for n in 10..20 {
            println!("t2: {}", n);
        }
    });

    t1.join().unwrap();
    t2.join().unwrap();
}
