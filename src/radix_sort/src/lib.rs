#[derive(Default)]
struct SortStats {
    comparisons: usize,
    moves: usize,
}

pub struct RadixSort {
    stats: SortStats,
}

impl RadixSort {
    pub fn new() -> Self {
        Self {
            stats: SortStats::default(),
        }
    }

    pub fn get_stats(&self) -> (usize, usize) {
        (self.stats.comparisons, self.stats.moves)
    }

    pub fn reset_stats(&mut self) {
        self.stats = SortStats::default();
    }

    pub fn sort<T>(&mut self, data: &mut [T])
    where
        T: Copy + RadixSortable + PartialOrd + Default + 'static,
    {
        if data.is_empty() {
            return;
        }

        // Reset stats for new sort operation
        self.reset_stats();

        let max_digits = data
            .iter()
            .map(|x| {
                self.stats.comparisons += 1; // Comparing during max operation
                x.max_digits()
            })
            .max()
            .unwrap_or(0);

        let mut temp = vec![T::default(); data.len()];
        let mut count = vec![0; 10];

        for place in 0..max_digits {
            count.fill(0);

            // Count frequencies
            for &num in data.iter() {
                let digit = num.get_digit(place);
                count[digit] += 1;
            }

            // Calculate cumulative count
            for i in 1..10 {
                count[i] += count[i - 1];
            }

            // Build output array
            for &num in data.iter().rev() {
                let digit = num.get_digit(place);
                count[digit] -= 1;
                temp[count[digit]] = num;
                self.stats.moves += 1; // Track element movement
            }

            // Copy back to original array
            data.copy_from_slice(&temp);
            self.stats.moves += data.len(); // Track bulk move operation
        }

        // Final step: handle negative numbers for integers if needed
        if std::any::TypeId::of::<T>() == std::any::TypeId::of::<i32>() {
            data.sort_by(|a, b| {
                self.stats.comparisons += 1;
                a.partial_cmp(b).unwrap()
            });
        }

        println!(
            "Radix Sort Stats - Comparisons: {}, Moves: {}",
            self.stats.comparisons, self.stats.moves
        );
    }
}

pub trait RadixSortable {
    fn get_digit(&self, place: usize) -> usize;
    fn max_digits(&self) -> usize;
}

impl RadixSortable for i32 {
    fn get_digit(&self, place: usize) -> usize {
        let abs_num = self.abs();
        (abs_num / 10_i32.pow(place as u32)) as usize % 10
    }

    fn max_digits(&self) -> usize {
        if *self == 0 {
            return 1;
        }
        let abs_num = self.abs();
        (abs_num as f64).log10().floor() as usize + 1
    }
}

impl RadixSortable for f64 {
    fn get_digit(&self, place: usize) -> usize {
        let abs_num = self.abs();
        let scaled = (abs_num * 1e6) as u64; // Scale up to handle 6 decimal places
        (scaled / 10_u64.pow(place as u32)) as usize % 10
    }

    fn max_digits(&self) -> usize {
        if *self == 0.0 {
            return 1;
        }
        let scaled = (self.abs() * 1e6) as u64; // Scale up to handle 6 decimal places
        (scaled as f64).log10().floor() as usize + 1
    }
}

impl RadixSortable for &str {
    fn get_digit(&self, place: usize) -> usize {
        if let Ok(num) = self.parse::<f64>() {
            let scaled = (num.abs() * 1e6) as u64;
            (scaled / 10_u64.pow(place as u32)) as usize % 10
        } else {
            if let Some(c) = self.chars().rev().nth(place) {
                (c as u8) as usize
            } else {
                0
            }
        }
    }

    fn max_digits(&self) -> usize {
        if let Ok(num) = self.parse::<f64>() {
            let scaled = (num.abs() * 1e6) as u64;
            if scaled == 0 {
                return 1;
            }
            (scaled as f64).log10().floor() as usize + 1
        } else {
            self.len()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integer_sort() {
        let mut arr = vec![170, -45, 75, 90, -802, 24, 2, 66];
        let mut sorter = RadixSort::new();
        sorter.sort(&mut arr);
        assert_eq!(arr, vec![-802, -45, 2, 24, 66, 75, 90, 170]);
    }

    #[test]
    fn test_float_sort() {
        let mut arr = vec![64.5, 34.0, 25.5, 12.2, 22.7, 11.1, 90.0];
        let mut sorter = RadixSort::new();
        sorter.sort(&mut arr);
        assert_eq!(arr, vec![11.1, 12.2, 22.7, 25.5, 34.0, 64.5, 90.0]);
    }

    #[test]
    fn test_string_sort() {
        let mut arr = vec!["170", "45", "7", "90", "802", "24", "2", "66"];
        let mut sorter = RadixSort::new();
        sorter.sort(&mut arr);
        assert_eq!(arr, vec!["2", "7", "24", "45", "66", "90", "170", "802"]);
    }
}
