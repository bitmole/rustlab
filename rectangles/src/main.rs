#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    let square = Rectangle::square(3);

    println!("The rect is {:?}", rect);
    println!("The square is {:?}", square);

    println!("The area of the rectangle is {} square pixels.", rect.area());
}
