use bucket_sort::BucketSort;
use criterion::BenchmarkId;
use criterion::Criterion;
use csv::ReaderBuilder;
use radix_sort::RadixSort;
use std::error::Error;

fn load_csv_column(file_path: &str, column_name: &str) -> Result<Vec<f64>, Box<dyn Error>> {
    let mut reader = ReaderBuilder::new()
        .delimiter(b',') // Specify the delimiter if necessary
        .from_path(file_path)?;
    let headers = reader.headers()?.clone();

    let mut column_data = Vec::new();

    for result in reader.records() {
        let record = result?;

        let column_index = headers
            .iter()
            .position(|header| header == column_name)
            .ok_or_else(|| format!("Column '{}' not found", column_name))?;
        if let Some(value) = record.get(column_index) {
            if let Ok(num) = value.parse::<i32>() {
                column_data.push(num as f64);
            }
            if let Ok(num) = value.parse::<f64>() {
                column_data.push(num);
            }
        }
    }

    Ok(column_data)
}
fn main() -> Result<(), Box<dyn Error>> {
    let bucket_sort = BucketSort;
    let radix_sort = RadixSort;
    let c: &mut Criterion = &mut Default::default();

    // Load Hotel Inventory Dataset
    let full_data: Vec<f64> =
        load_csv_column("Hotel_Item_Inventory_Dataset.csv", "Purchase_Price").unwrap();

    // Test sizes
    let sizes = [100, 500, 1000];

    // Unordered sort benchmark
    for &size in &sizes {
        println!("Unordered sort benchmark for size: {}", size);
        // Get subset of data for current size
        let unordered_data: Vec<f64> = full_data[0..size].to_vec();

        // Benchmark Bucket Sort
        c.bench_with_input(
            BenchmarkId::new("Bucket Sort", size),
            &unordered_data,
            |b, data| {
                b.iter(|| {
                    let mut data_copy = data.clone();
                    bucket_sort.sort(&mut data_copy);
                });
            },
        );

        // Benchmark Radix Sort
        c.bench_with_input(
            BenchmarkId::new("Radix Sort", size),
            &unordered_data,
            |b, data| {
                b.iter(|| {
                    let mut data_copy = data.clone();
                    radix_sort.sort(&mut data_copy);
                });
            },
        );
    }

    // Nearly ordered sort benchmark
    for &size in &sizes {
        println!("Nearly ordered sort benchmark for size: {}", size);
        // Get subset of data for current size
        let mut partial_sorted: Vec<f64> = full_data[0..size].to_vec();
        partial_sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        // Swap some elements to make it nearly sorted
        for i in (0..size).step_by(5) {
            if i + 1 < size {
                partial_sorted.swap(i, i + 1);
            }
        }

        // Benchmark Bucket Sort
        c.bench_with_input(
            BenchmarkId::new("Bucket Sort", size),
            &partial_sorted,
            |b, data| {
                b.iter(|| {
                    let mut data_copy = data.clone();
                    bucket_sort.sort(&mut data_copy);
                });
            },
        );

        // Benchmark Radix Sort
        c.bench_with_input(
            BenchmarkId::new("Radix Sort", size),
            &partial_sorted,
            |b, data| {
                b.iter(|| {
                    let mut data_copy = data.clone();
                    radix_sort.sort(&mut data_copy);
                });
            },
        );
    }

    Ok(())
}
