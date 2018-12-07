fn main() {
    let width = 30;
    let height = 50;

    println!("Area is {}", area(width, height));

    let rect1 = (30, 50);
    println!("Area is {}", area2(rect1));

    let rect2 = Rectangle {width: 30, height: 50};
    println!("Rectangle is {:#?}", rect2);
    println!("Area is {}", area3(&rect2));
    println!("Area is {}", rect2.area());
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn _can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height 
    }

    fn _square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}
