use bucket_sort::BucketSort;
use radix_sort::RadixSort;
use std::time::Instant;

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
}
