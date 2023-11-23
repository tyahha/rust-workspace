use std::thread;
use std::time::Duration;

pub fn fearless_concurrency_main() {
    thread_spawn_sample();
}

fn thread_spawn_sample() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    let v = vec![1,2,3];
    let handle2 = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
    handle2.join().unwrap();
}
