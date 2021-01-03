#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}


struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iterator_sum() {
        let vec = vec![1, 2, 3];
        let result: i32 = vec.iter().sum();
        assert_eq!(result, 6)
    }

    #[test]
    fn iterator_map() {
        let vec = vec![1, 2, 3];

        let result: Vec<i32> = vec.iter().map(|x| x + 1).collect();

        assert_eq!(vec![2, 3, 4], result);

        assert_eq!(vec, vec![1, 2, 3]);
    }

    #[test]
    fn iterator_filter() {
        let v1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 13, 14];

        let bigger_than_7: Vec<i32> = v1.into_iter().filter(|&x| x > 7).collect();
        assert_eq!(bigger_than_7, vec![8, 9, 10, 11, 12, 13, 13, 14]);
    }

    #[test]
    fn filter_custom_type() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let expected = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let custom_size: Vec<Shoe> = shoes.into_iter().filter(|s| s.size == 10).collect();
        assert_eq!(custom_size, expected);
    }

    #[test]
    fn next_directly_counter() {
        let mut counter = Counter::new();
        for n in 1..6 {
            assert_eq!(counter.next(), Some(n))
        }
    }

    #[test]
    fn other_iterator_trait_methods_in_counter() {
        let sum: u32 = Counter::new()
            .zip(Counter::new().skip(1))
            .map(|(a, b)| a * b)
            .filter(|x| x % 3 == 0)
            .sum();

        assert_eq!(18, sum)
    }
}
