pub struct BucketSort;

pub trait Sortable {
    fn to_bucket_index(&self, bucket_count: usize, max_value: &Self) -> usize;
    fn get_bucket_count(&self) -> usize;
}

impl Sortable for f64 {
    fn to_bucket_index(&self, bucket_count: usize, max_value: &Self) -> usize {
        ((self / max_value) * (bucket_count - 1) as f64).floor() as usize
    }

    fn get_bucket_count(&self) -> usize {
        self.sqrt().ceil() as usize
    }
}

impl Sortable for i32 {
    fn to_bucket_index(&self, bucket_count: usize, max_value: &Self) -> usize {
        ((*self as f64 / *max_value as f64) * (bucket_count - 1) as f64).floor() as usize
    }

    fn get_bucket_count(&self) -> usize {
        (*self as f64).sqrt().ceil() as usize
    }
}

impl<'a> Sortable for &'a str {
    fn to_bucket_index(&self, bucket_count: usize, _: &Self) -> usize {
        // Special handling for numeric strings
        if self.chars().next().map_or(false, |c| c.is_digit(10)) {
            // Parse the string as f64 for numeric comparison
            if let Ok(num) = self.parse::<f64>() {
                let max_value = 1000.0; // Use a fixed max value for normalization
                return ((num / max_value) * (bucket_count - 1) as f64).floor() as usize;
            }

            if let Ok(num) = self.parse::<u32>() {
                let max_value = 1000.0; // Use a fixed max value for normalization
                return ((num as f64 / max_value) * (bucket_count - 1) as f64).floor() as usize;
            }
        }

        // Fall back to alphabetical sorting for non-numeric strings
        if let Some(first_char) = self.chars().next() {
            ((first_char as u32) % bucket_count as u32) as usize
        } else {
            0
        }
    }

    fn get_bucket_count(&self) -> usize {
        // More buckets for numeric strings
        if self.chars().next().map_or(false, |c| c.is_digit(10)) {
            50 // Reduced bucket count for better distribution
        } else {
            26 // Default for alphabetical strings
        }
    }
}

impl BucketSort {
    pub fn sort<T>(&self, data: &mut [T])
    where
        T: Copy + PartialOrd + Sortable,
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

        for &value in data.iter() {
            let bucket_index = value.to_bucket_index(bucket_count, &max_value);
            buckets[bucket_index].push(value);
        }

        for bucket in buckets.iter_mut() {
            bucket.sort_by(|a, b| a.partial_cmp(b).unwrap());
        }

        let mut index = 0;
        for bucket in buckets {
            for &value in bucket.iter() {
                data[index] = value;
                index += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integer_sort() {
        let mut arr = vec![64, 34, 25, 12, 22, 11, 90];
        let sorter = BucketSort;
        sorter.sort(&mut arr);
        assert_eq!(arr, vec![11, 12, 22, 25, 34, 64, 90]);
    }

    #[test]
    fn test_float_sort() {
        let mut arr = vec![64.5, 34.0, 25.5, 12.2, 22.7, 11.1, 90.0];
        let sorter = BucketSort;
        sorter.sort(&mut arr);
        assert_eq!(arr, vec![11.1, 12.2, 22.7, 25.5, 34.0, 64.5, 90.0]);
    }

    #[test]
    fn test_str_char_sort() {
        let mut arr = vec!["dog", "cat", "bird", "ant"];
        let sorter = BucketSort;
        sorter.sort(&mut arr);
        assert_eq!(arr, vec!["ant", "bird", "cat", "dog"]);
    }

    #[test]
    fn test_str_floats_sort() {
        let mut arr = vec!["170.7", "45.7", "7.5", "9.0", "80.2", "2.4", "2.7", "6.6"];
        let sorter = BucketSort;
        sorter.sort(&mut arr);
        assert_eq!(
            arr,
            vec!["2.4", "2.7", "6.6", "7.5", "9.0", "45.7", "80.2", "170.7"]
        );
    }

    #[test]
    fn test_str_integers_sort() {
        let mut arr = vec!["170", "45", "7", "9", "80", "2", "2", "6"];
        let sorter = BucketSort;
        sorter.sort(&mut arr);
        assert_eq!(arr, vec!["2", "2", "6", "7", "9", "45", "80", "170"]);
    }
}
