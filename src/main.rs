use std::thread;
use std::time::Instant;

const SIZE: usize = 1000;
const NUM_THREADS: usize = 8;

fn main() {
    let a = vec![vec![1.0; SIZE]; SIZE];
    let b = vec![vec![1.0; SIZE]; SIZE];
    let result = vec![vec![0.0; SIZE]; SIZE];

    let start = Instant::now();

    let mut handles = Vec::new();
    let chunk_size = SIZE / NUM_THREADS;

    for thread_id in 0..NUM_THREADS {
        let a_clone = a.clone();
        let b_clone = b.clone();
        let mut result_slice = result.clone();

        let handle = thread::spawn(move || {
            let start_row = thread_id * chunk_size;
            let end_row = if thread_id == NUM_THREADS - 1 {
                SIZE
            } else {
                (thread_id + 1) * chunk_size
            };

            for i in start_row..end_row {
                for j in 0..SIZE {
                    for k in 0..SIZE {
                        result_slice[i][j] += a_clone[i][k] * b_clone[k][j];
                    }
                }
            }

            result_slice
        });

        handles.push(handle);
    }

    // Saml resultaterne (kun nødvendigt hvis du skal bruge result senere)
    let mut final_result = vec![vec![0.0; SIZE]; SIZE];
    for handle in handles {
        let partial = handle.join().unwrap();
        for i in 0..SIZE {
            for j in 0..SIZE {
                final_result[i][j] += partial[i][j];
            }
        }
    }

    let duration = start.elapsed();
    println!("Time taken (multi-threaded): {:?}", duration);
}