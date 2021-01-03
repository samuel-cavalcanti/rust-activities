use std::thread;
use std::time::Duration;

struct Cacher<T>
    where T: Fn(u32) -> u32, {
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
    where T: Fn(u32) -> u32, {
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn simulated_Expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");

    thread::sleep(Duration::from_secs(2));

    intensity
}


fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_closure = Cacher::new(simulated_Expensive_calculation);


    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure.value(intensity));
        println!("Next, do {} situps", expensive_closure.value(intensity));
    } else {
        println!("Today, run for {} minutes", expensive_closure.value(intensity))
    }
}

fn main() {
    generate_workout(21, 0);
}
