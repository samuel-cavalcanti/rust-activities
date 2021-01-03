#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn set_width(&mut self, width: u32) {
        self.width = width
    }

    fn can_hold(&self, another_rectangle: &Rectangle) -> bool {
        self.width > another_rectangle.width && self.height > another_rectangle.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let mut rect = Rectangle {
        width: 30,
        height: 20,
    };

    let rect_2 = Rectangle {
        width: 25,
        height: 10,
    };

    let square = Rectangle::square(10);

    println!(" the area of the rectangle is {} square pixels", rect.area());

    println!("rect is {:#?}", rect);

    println!("rect1 can hold ? {}", rect.can_hold(&rect_2));

    rect.set_width(20);

    println!("rect1 can hold ? {}", rect.can_hold(&rect_2));

    println!(" the area of the rectangle is {} square pixels", rect.area());
    println!("rect is {:#?}", rect);

    println!("square is {:#?}", square);
}
