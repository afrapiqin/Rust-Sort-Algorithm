use bucket_sort::BucketSort;
use radix_sort::RadixSort;
use std::time::Instant;
use criterion::BenchmarkId;
use criterion::Criterion;
use rand::prelude::*;

fn main() {
    let bucket_sort = BucketSort;
    let radix_sort = RadixSort;

    // Integer sorting using Bucket Sort
    let mut data = vec![170, 45, 75, 90, 802, 24, 2, 66];
    let start = Instant::now();
    bucket_sort.sort(&mut data);
    let duration = start.elapsed();
    println!("Bucket sort (integers): {data:?} took: {:?}", duration);

    // Integer sorting using Radix Sort
    let mut data = vec![170, 45, 75, 90, 802, 24, 2, 66];
    let start = Instant::now();
    radix_sort.sort(&mut data);
    let duration = start.elapsed();
    println!("Radix sort (integers): {data:?} took: {:?}", duration);

    // Float sorting using Bucket Sort
    let mut data = vec![170.7, 45.7, 7.5, 9.0, 80.2, 2.4, 2.7, 6.6];
    let start = Instant::now();
    bucket_sort.sort(&mut data);
    let duration = start.elapsed();
    println!("Bucket sort (floats): {data:?} took: {:?}", duration);

    // Float sorting using Radix Sort
    let mut data = vec![170.7, 45.7, 7.5, 9.0, 80.2, 2.4, 2.7, 6.6];
    let start = Instant::now();
    radix_sort.sort(&mut data);
    let duration = start.elapsed();
    println!("Radix sort (floats): {data:?} took: {:?}", duration);

    // String sorting using Bucket Sort
    let mut data = vec!["170.7", "45.7", "7.5", "9.0", "80.2", "2.4", "2.7", "6.6"];
    let start = Instant::now();
    bucket_sort.sort(&mut data);
    let duration = start.elapsed();
    println!("Bucket sort (strings): {data:?} took: {:?}", duration);

    // String sorting using Radix Sort
    let mut data = vec!["170.7", "45.7", "7.5", "9.0", "80.2", "2.4", "2.7", "6.6"];
    let start = Instant::now();
    radix_sort.sort(&mut data);
    let duration = start.elapsed();
    println!("Radix sort (strings): {data:?} took: {:?}", duration);

    //Benchmarking integers
    let datas = vec![170, 45, 75, 90, 802, 24, 2, 66];
    let start = Instant::now();
    let duration = start.elapsed();
    println!("Starting benchmark sort (integers): {datas:?} ");
    let c: &mut Criterion = &mut Default::default();
    for size in datas {
        let mut rng = rand::thread_rng();

        // Benchmark Bucket Sort
        let bucket_data: Vec<i32> = (0..size).map(|_| rng.gen_range(0..1000)).collect();
        c.bench_with_input(BenchmarkId::new("Bucket Sort (i32)", size), &bucket_data, |b, data| {
            b.iter(|| {
                let mut data_copy = data.clone();
                bucket_sort.sort(&mut data_copy);
            });
        });

        // Benchmark Radix Sort
        let radix_data: Vec<i32> = (0..size).map(|_| rng.gen_range(0..1000)).collect();
        c.bench_with_input(BenchmarkId::new("Radix Sort (i32)", size), &radix_data, |b, data| {
            b.iter(|| {
                let mut data_copy = data.clone(); // Important: Clone data for each iteration
                radix_sort.sort(&mut data_copy);
            });
        });
    }
    println!("Benchmarking (integers): took: {:?}", duration);

    //Benchmarking floats
    let data_input_1: Vec<f64> = vec![170.0, 45.0, 75.0, 90.0, 802.0, 24.0, 2.0, 66.0];
    let sizes = vec![data_input_1.len()]; // Only one size now
    let start = Instant::now();
    let duration = start.elapsed();
    println!("Starting benchmark sort (floats): {data_input_1:?} ");
    let c: &mut Criterion = &mut Default::default();
    for size in sizes {
        // Benchmark Bucket Sort (f64)
        c.bench_with_input(BenchmarkId::new("Bucket Sort (f64)", size), &data_input_1, |b, data| {
            b.iter(|| {
                let mut data_copy = data.clone();
                bucket_sort.sort(&mut data_copy);
            });
        });

        // Benchmark Radix Sort (f64)
        c.bench_with_input(BenchmarkId::new("Radix Sort (f64)", size), &data_input_1, |b, data| {
            b.iter(|| {
                let mut data_copy = data.clone();
                radix_sort.sort(&mut data_copy);
            });
        });
    }
    println!("Benchmarking (floats): took: {:?}", duration);

    //Benchmarking strings
    /*
    let data_input_2: Vec<String> = vec!["170.7", "45.7", "7.5", "9.0", "80.2", "2.4", "2.7", "6.6"]
        .iter()
        .map(|s| s.to_string())
        //.copied()
        .collect();
    let sizes = vec![data_input_2.len()];
    let start = Instant::now();
    let duration = start.elapsed();
    println!("Starting benchmark sort (strings): {data_input_2:?} ");
    let c: &mut Criterion = &mut Default::default();
    for size in sizes {
        // Benchmark Radix Sort (String)
        c.bench_with_input(BenchmarkId::new("Radix Sort (String)", size), &data_input_2, |b, data| {
            b.iter(|| {
                let mut data_copy = data.clone();
                radix_sort.sort(&mut data_copy);
            });
        });

        // Benchmark Bucket Sort (String)
        c.bench_with_input(BenchmarkId::new("Bucket Sort (String)", size), &data_input_2, |b, data| {
            b.iter(|| {
                let mut data_copy = data.clone();
                bucket_sort.sort(&mut data_copy);
            });
        });
    }
    println!("Benchmarking (strings): took: {:?}", duration);
     */
}
