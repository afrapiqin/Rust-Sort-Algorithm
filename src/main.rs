use bucket_sort::BucketSort;
use std::time::Instant;

fn main() {
    let bucket_sort = BucketSort;

    let mut data = vec![170, 45, 75, 90, 802, 24, 2, 66];
    let start = Instant::now();
    bucket_sort.sort(&mut data);
    let duration = start.elapsed();
    println!("Bucket sort {data:?} took: {:?}", duration);

    let mut data = vec![170.7, 45.7, 7.5, 9.0, 80.2, 2.4, 2.7, 6.6];

    let start = Instant::now();
    bucket_sort.sort(&mut data);
    let duration = start.elapsed();
    println!("Bucket sort {data:?} took: {:?}", duration);

    let mut data = vec!["170.7", "45.7", "7.5", "9.0", "80.2", "2.4", "2.7", "6.6"];

    let start = Instant::now();
    bucket_sort.sort(&mut data);
    let duration = start.elapsed();
    println!("Bucket sort {data:?} took: {:?}", duration);
}
