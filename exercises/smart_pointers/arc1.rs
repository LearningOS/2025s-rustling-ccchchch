#![forbid(unused_imports)] // Do not change this, (or the next) line.
use std::sync::Arc;
use std::thread;

fn main() {
    let numbers: Vec<_> = (0..100u32).collect();
    
    // Wrap the numbers Vec in an Arc so it can be shared safely across threads.
    let shared_numbers = Arc::new(numbers);
    
    let mut joinhandles = Vec::new();

    for offset in 0..8 {
        // Clone the Arc for each thread
        let child_numbers = Arc::clone(&shared_numbers);
        
        joinhandles.push(thread::spawn(move || {
            // In each thread, filter based on the offset and sum the appropriate numbers
            let sum: u32 = child_numbers.iter()
                .filter(|&&n| n % 8 == offset)
                .sum();
            println!("Sum of offset {} is {}", offset, sum);
        }));
    }

    // Join all the threads
    for handle in joinhandles.into_iter() {
        handle.join().unwrap();
    }
}
