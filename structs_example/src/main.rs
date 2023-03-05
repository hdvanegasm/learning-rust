#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            height: size,
            width: size
        }
    }
}

fn main() {
    let rect = Rectangle {
        height: 43,
        width: 23
    };

    // Sending reference will allow to main to
    // keep the ownership of the rect variable.
    println!("The area of the rectangle is {}", 
        rect.area()
    );

    println!("The rectangle is {:#?}", rect);

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let rect_square = Rectangle::square(3);
    println!("Dimensions of square: {:?}", rect_square);
}
