# Design Documentation: Sorting Algorithm Implementation in Rust

## 1. Project Structure
```
sort_algorithm/
├── src/
│   ├── main.rs
│   ├── bucket_sort/
│   │   ├── Cargo.toml
│   │   └── src/lib.rs
│   └── radix_sort/
│       ├── Cargo.toml
│       └── src/lib.rs
└── Cargo.toml
```

## 2. Sorting Implementations

### Bucket Sort Implementation
- Located in `bucket_sort/src/lib.rs`
- Key features:
  - Generic implementation supporting multiple types
  - Custom trait `BucketSortable` for type-specific behavior
  - Dynamic bucket sizing based on data characteristics
  - Specialized handling for different data types:
    - Floating-point numbers: Normalized bucketing using relative scaling
    - Integers: Direct mapping with range-based distribution
    - Strings: Dual-mode handling (numeric vs alphabetical)
- Implementation Strategy:
  1. Calculate optimal bucket count based on input size
  2. Distribute elements into buckets using normalized indices
  3. Sort individual buckets using stable sorting
  4. Concatenate sorted buckets
- Key Implementation:
```rust
impl BucketSort {
    pub fn sort<T>(&self, data: &mut [T])
    where
        T: Copy + PartialOrd + BucketSortable,
    {
        if data.is_empty() {
            return;
        }

        let max_value = *data
            .iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();
        let bucket_count = max_value.get_bucket_count();
        let mut buckets: Vec<Vec<T>> = vec![Vec::new(); bucket_count];

        // Distribution phase
        for &value in data.iter() {
            let bucket_index = value.to_bucket_index(bucket_count, &max_value);
            buckets[bucket_index].push(value);
        }

        // Sorting phase
        for bucket in buckets.iter_mut() {
            bucket.sort_by(|a, b| a.partial_cmp(b).unwrap());
        }

        // Collection phase
        let mut index = 0;
        for bucket in buckets {
            for &value in bucket.iter() {
                data[index] = value;
                index += 1;
            }
        }
    }
}
```

### Radix Sort Implementation
- Located in `radix_sort/src/lib.rs`
- Key features:
  - LSD (Least Significant Digit) approach
  - Base-10 implementation for natural number representation
  - Custom digit extraction for different types
  - Stable sorting within each digit position
- Implementation Strategy:
  1. Determine maximum number of digits in dataset
  2. Process digits from least to most significant
  3. Use counting sort for each digit position
  4. Maintain stability through ordered reassembly
- Key Implementation:
```rust
impl RadixSort {
    pub fn sort<T>(&self, data: &mut [T])
    where
        T: Copy + RadixSortable,
    {
        if data.is_empty() {
            return;
        }

        // Determine maximum digits
        let max_digits = data.iter().map(|item| item.max_digits()).max().unwrap_or(0);
        let mut buckets: Vec<Vec<T>> = vec![Vec::new(); 10];

        // Process each digit position
        for place in 0..max_digits {
            // Distribution by current digit
            for &value in data.iter() {
                let digit = value.get_digit(place);
                buckets[digit].push(value);
            }

            // Collection phase
            let mut index = 0;
            for bucket in buckets.iter_mut() {
                for &value in bucket.iter() {
                    data[index] = value;
                    index += 1;
                }
                bucket.clear();
            }
        }
    }
}
```

## 3. Traits

### BucketSortable Trait
```rust
pub trait BucketSortable {
    fn to_bucket_index(&self, bucket_count: usize, max_value: &Self) -> usize;
    fn get_bucket_count(&self) -> usize;
}
```
- Purpose:
  - Enables generic bucket sort implementation
  - Allows type-specific bucket distribution logic
- Key Methods:
  - `to_bucket_index`: Maps elements to appropriate buckets
  - `get_bucket_count`: Determines optimal bucket count for type
- Implementations:
  - f64: Uses normalized floating-point arithmetic
  - i32: Integer-based mapping with range scaling
  - &str: Hybrid approach for numeric and alphabetic strings

### RadixSortable Trait
```rust
pub trait RadixSortable {
    fn get_digit(&self, place: usize) -> usize;
    fn max_digits(&self) -> usize;
}
```
- Purpose:
  - Provides digit-wise access to elements
  - Supports different numeric representations
- Key Methods:
  - `get_digit`: Extracts digit at specified position
  - `max_digits`: Determines number of digits in element
- Implementations:
  - i32: Base-10 digit extraction
  - f64: Scaled integer conversion for digit access
  - &str: Character-based or numeric string handling

## 4. Performance Analysis

### Time Complexity Analysis
- Bucket Sort:
  - Best Case: Ω(n + k) - Uniform distribution
  - Average Case: Θ(n + k) - Random distribution
  - Worst Case: O(n²) - When elements cluster in single bucket
  - Where: n = number of elements, k = number of buckets

- Radix Sort:
  - Consistent Case: Θ(d * (n + k))
  - Where: d = number of digits, n = elements, k = base (10)

### Space Complexity
- Bucket Sort:
  - O(n + k) for bucket array
  - Additional O(n) for temporary storage
  - Dynamic memory allocation based on bucket count

- Radix Sort:
  - O(n) for auxiliary array
  - O(k) for counting array
  - Constant extra space for digit processing

## 6. Dependencies
```toml
[dependencies]
bucket_sort = { path = "src/bucket_sort" }
radix_sort = { path = "src/radix_sort" }
criterion = "0.3"
csv = "1.1"
```
