use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let mut handles = vec![];
    for i in 0..10 {
        handles.push(thread::spawn(move || {
            let start = Instant::now();
            thread::sleep(Duration::from_millis(250));
            println!("thread {} is complete", i);
            start.elapsed().as_millis() // return the elapsed time in milliseconds
        }));
    }

    let mut results: Vec<u128> = vec![];
    for handle in handles {
        // Wait for the thread to complete and get the result
        let result = handle.join().unwrap(); // `unwrap` is safe here because we know the thread won't panic
        results.push(result); // Collect the result
    }

    if results.len() != 10 {
        panic!("Oh no! All the spawned threads did not finish!");
    }

    println!();
    for (i, result) in results.into_iter().enumerate() {
        println!("thread {} took {}ms", i, result);
    }
}
