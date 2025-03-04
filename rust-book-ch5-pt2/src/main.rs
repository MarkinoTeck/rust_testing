#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }
}

impl Rectangle {
    // you can do this yay.
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!("{:#?}", rect);

    let rect1 = Rectangle {
        width: 20,
        height: 10,
    };

    let rect2 = Rectangle {
        width: 50,
        height: 100,
    };

    let rect3 = Rectangle::square(10);
    println!("square: {:#?}", rect3);

    println!(
        "The area if the rectangle is {} square pixels.",
        rect.area()
    );

    println!(
        "rect can hold:\n rect1: {0}\n rect1: {1}",
        rect.can_hold(&rect1),
        rect.can_hold(&rect2),
    );
}
