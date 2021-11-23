#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        return self.width * self.height
    }

    fn width(&self) -> bool {
        return self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        return self.width > other.width && self.height > other.height;
    }

    fn square(size: u32) -> Rectangle {
        return Rectangle {
            width: size,
            height: size
        };
    }
}

fn main() {
    let rectangle = Rectangle {
        width: 30,
        height: 50
    };

    println!("The area of the rectangle is {} square pixels.", rectangle.area());

    println!("rectangle is {:#?}", rectangle);

    dbg!(&rectangle);

    if rectangle.width() {
        println!("The rectangle has a nonzero width; it is {}", rectangle.width);
    }

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

    let square1 = Rectangle::square(10);
    println!("Square height size is {}", square1.height);
    println!("Square width size is {}", square1.width);
    println!("Square area is {}", square1.area());

}

