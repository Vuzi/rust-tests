extern crate rand;
extern crate chrono;

use rand::Rng;
use chrono::*;

// Naive bubble sort :)
fn bubble_sort<T: Ord + Clone>(values: &mut[T]) {
    loop {
        let mut swapped = false;

        for i in 0..(values.len() - 1) {
            if values[i] > values[i +1] {
                values.swap(i, i + 1);
                swapped = true;
            }
        }
    
        if !swapped { break; } // bubble sort done
    }
}

fn random_i32() -> i32 {
    rand::thread_rng().gen::<i32>()
}

// Test the provided sort method
fn test_sort<T: Ord + Clone>(
    number_test: u32,
    size_test:usize,
    generator: fn() -> T,
    sorter: fn(&mut[T])
) {
    let mut test_duration: Duration = Duration::zero();

    // For each test to run..
    for _ in 0..number_test {
        let mut values = vec![generator(); size_test];

        // Array init
        for i in 0..values.len() {
            values[i] = generator();
        }

        // Test sort for this array
        let before = UTC::now(); 
        sorter(values.as_mut_slice());
        let after = UTC::now();

        test_duration = test_duration + (after - before)
    }
    
    // Show result
    println!("Sort {} arrays of {} elements in {}ms",
        number_test, size_test, test_duration.num_milliseconds());
}

// Sort using the provided sorting method
fn sort<T: Ord + Clone>(values: &mut[T], sorter: fn(&mut[T])) {
    let before = UTC::now(); 
    sorter(values);
    let after = UTC::now();
    println!("Sort in {:?}", after - before);
}

fn main() {
    println!("Test bubble sort =>");

    test_sort(1000, 256, random_i32, bubble_sort);
}
