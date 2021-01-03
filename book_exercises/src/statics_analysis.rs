pub mod statics_analysis {
    use std::{collections::HashMap,vec};

    fn get_middle_index(size: usize) -> usize {
        if size % 2 == 0 {
            return size / 2;
        } else {
            return size / 2 + 1;
        }
    }
    pub fn statics_analysis_from_array(mut array: vec::Vec<i32>) {
        let mut mean = 0.0;
        let size = array.len() as f32;
        let middle = get_middle_index(size as usize);
        let mut map = HashMap::new();
        let mut mode_value = 0;
        let mut mode_frequency = 0;
        array.sort();
        let median = array[middle];
        for value in array {
            mean += value as f32;
            let count = map.entry(value).or_insert(0);
            *count += 1;
            if *count > mode_frequency {
                mode_value = value;
                mode_frequency = *count;
            }
        }
        mean = mean / size;
        println!("mean of vector {}", mean);
        println!("median of vector {}", median);
        println!("mode of vector {}", mode_value);
    }
}