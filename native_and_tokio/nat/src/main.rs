use rand::Rng;
use tokio;

use std::time::{Instant};

async fn generate_numbers_native() -> Vec<u32> {
    let now = Instant::now();
    let mut rng = rand::thread_rng();
    let numbers: Vec<u32> = (0..5000000).map(|_| rng.gen_range(1000000..2_000_000)).collect();
    let later = Instant::now();
    println!("native create{:?}", later.duration_since(now));
    numbers
}

async fn generate_numbers_tokio() -> Vec<u32> {
    let now = Instant::now();
    let mut rng = rand::thread_rng();
    let numbers: Vec<u32> = (0..5000000).map(|_| rng.gen_range(1000000..2_000_000)).collect();
    let later = Instant::now();
    println!("tokio create{:?}", later.duration_since(now));
    numbers
}

async fn sort_numbers_native(mut numbers: Vec<u32>) -> Vec<u32> {
    let now = Instant::now();
    numbers.sort();
    let later = Instant::now();
    println!("native sort{:?}", later.duration_since(now));
    numbers
}

async fn sort_numbers_tokio(mut numbers: Vec<u32>) -> Vec<u32> {
    let now = Instant::now();
    numbers.sort();
    let later = Instant::now();
    println!("tokio sort{:?}", later.duration_since(now));
    numbers
}

// Funktion zur Zeitmessung
async fn measure_time<F, T>(task: F) -> (T, u128)
where
    F: FnOnce() -> T,
{
    let start = Instant::now();
    let result = task();
    let duration = start.elapsed().as_millis();
    (result, duration)
}

#[tokio::main]
async fn main() {
    // Native async function for generating and sorting numbers
    let (sorted_native, native_duration) = measure_time(|| async {
        let numbers_native = generate_numbers_native().await;
        sort_numbers_native(numbers_native).await
    }).await;

    // Tokio async function for generating and sorting numbers
    let (sorted_tokio, tokio_duration) = measure_time(|| async {
        let numbers_tokio = generate_numbers_tokio().await;
        sort_numbers_tokio(numbers_tokio).await
    }).await;

    // Ausgabe der ersten 10 Zahlen zur Veranschaulichung
    println!("Native Rust Sorted (First 10): {:?}", &sorted_native.await[..1000]);
    println!("Tokio Sorted (First 10): {:?}", &sorted_tokio.await[..1000]);

    // Ausgabe der benötigten Zeit in Millisekunden
    println!("Native Rust duration: {} ms", native_duration);
    println!("Tokio duration: {} ms", tokio_duration);
}
