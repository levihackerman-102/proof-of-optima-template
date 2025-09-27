#![no_main]
#![no_std]

extern crate alloc;

use alloc::vec;
use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

fn main() {
    let n: usize = env::read();

    let mut dist = vec![0u64; n * n];
    for i in 0..(n * n) {
        dist[i] = env::read();
    }

    env::commit(&dist);

    let mut tour = vec![0usize; n];
    for i in 0..n {
        tour[i] = env::read();
    }

    let mut visited = vec![false; n];
    for &city in &tour {
        if city >= n || visited[city] {
            panic!("Invalid tour: repeated or out-of-range city");
        }
        visited[city] = true;
    }

    let mut total = 0u64;
    for i in 0..n {
        let from = tour[i];
        let to = tour[(i + 1) % n];
        total = total.checked_add(dist[from * n + to]).expect("Distance overflow");
    }

    env::commit(&total);
}
