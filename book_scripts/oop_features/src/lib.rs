pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn new() -> AveragedCollection {
        AveragedCollection {
            list: vec![],
            average: 0.0,
        }
    }

    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average()
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    pub fn remove(&mut self, index: usize) -> Result<(), &'static str> {
        if index >= self.list.len() {
            return Err("index > average list");
        }
        self.list.remove(index);
        self.update_average();
        Ok(())
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        let total = total as f64;

        let size: f64 = self.list.len() as f64;

        self.average = if size == 0.0 { size } else { total / size };


    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_average() {
        let mut average = AveragedCollection::new();
        average.add(5);

        assert_eq!(average.average(), 5.0);

        average.add(10);

        assert_eq!(average.average(), (10.0 + 5.0) / 2.0);

        average.add(3);

        assert_eq!(average.average(), (10.0 + 5.0 + 3.0) / 3.0);
    }

    #[test]
    fn remove_average() {
        let mut average = AveragedCollection {
            average: (10.0 + 5.0 + 3.0) / 3.0,
            list: vec![5, 10, 3],
        };

        average.remove(2);
        assert_eq!(average.average(), (10.0 + 5.0) / 2.0);

        average.remove(1);
        assert_eq!(average.average(), 5.0);

        average.remove(0);
        assert_eq!(average.average(), 0.0);
    }
}