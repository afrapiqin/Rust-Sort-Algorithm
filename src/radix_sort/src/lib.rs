pub struct RadixSort;

pub trait RadixSortable {
    fn get_digit(&self, place: usize) -> usize;
    fn max_digits(&self) -> usize;
}

impl RadixSortable for i32 {
    fn get_digit(&self, place: usize) -> usize {
        (*self / 10_i32.pow(place as u32)) as usize % 10
    }

    fn max_digits(&self) -> usize {
        let mut num = *self;
        let mut digits = 0;
        while num > 0 {
            digits += 1;
            num /= 10;
        }
        digits
    }
}

impl RadixSortable for f64 {
    fn get_digit(&self, place: usize) -> usize {
        let scaled = (self * 10_f64.powi(place as i32)) as usize;
        scaled % 10
    }

    fn max_digits(&self) -> usize {
        let mut num = *self as usize;
        let mut digits = 0;
        while num > 0 {
            digits += 1;
            num /= 10;
        }
        digits
    }
}

impl RadixSortable for &str {
    fn get_digit(&self, place: usize) -> usize {
        // Handle numeric strings
        if let Ok(num) = self.parse::<f64>() {
            let scaled = (num * 10_f64.powi(place as i32)) as usize;
            return scaled % 10;
        }
        // Non-numeric strings are treated differently
        if let Some(char) = self.chars().nth(place) {
            return char as usize;
        }
        0
    }

    fn max_digits(&self) -> usize {
        if let Ok(num) = self.parse::<f64>() {
            let mut digits = 0;
            let mut value = num as usize;
            while value > 0 {
                digits += 1;
                value /= 10;
            }
            return digits;
        }
        self.len()
    }
}

impl RadixSort {
    pub fn sort<T>(&self, data: &mut [T])
    where
        T: Copy + RadixSortable,
    {
        if data.is_empty() {
            return;
        }

        // Determine the maximum number of digits in the dataset
        let max_digits = data.iter().map(|item| item.max_digits()).max().unwrap_or(0);

        let mut buckets: Vec<Vec<T>> = vec![Vec::new(); 10];

        for place in 0..max_digits {
            // Place items into buckets based on the current digit
            for &value in data.iter() {
                let digit = value.get_digit(place);
                buckets[digit].push(value);
            }

            // Collect items back into the main array
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integer_sort() {
        let mut arr = vec![170, 45, 75, 90, 802, 24, 2, 66];
        let sorter = RadixSort;
        sorter.sort(&mut arr);
        assert_eq!(arr, vec![2, 24, 45, 66, 75, 90, 170, 802]);
    }

    #[test]
    fn test_float_sort() {
        let mut arr = vec![64.5, 34.0, 25.5, 12.2, 22.7, 11.1, 90.0];
        let sorter = RadixSort;
        sorter.sort(&mut arr);
        assert_eq!(arr, vec![11.1, 12.2, 22.7, 25.5, 34.0, 64.5, 90.0]);
    }

    #[test]
    fn test_string_sort() {
        let mut arr = vec!["170", "45", "7", "90", "802", "24", "2", "66"];
        let sorter = RadixSort;
        sorter.sort(&mut arr);
        assert_eq!(arr, vec!["2", "7", "24", "45", "66", "90", "170", "802"]);
    }
}
