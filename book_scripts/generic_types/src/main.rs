fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![102.51, 34.213, 60.231, 89.33, 54.2, 2.1, 4.3, 0.8];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let mut result = largest(&char_list);
    println!("The largest number is {}", result);

    result = &'s';

    println!("new result is {}", result);
}
