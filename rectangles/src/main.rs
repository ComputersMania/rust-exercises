struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Associated functions
    fn square(l: u32) -> Rectangle {
        Rectangle {
            width: l,
            height: l,
        }
    }

    // Methods
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_contain(&self, contained: &Rectangle) -> bool {
        self.width > contained.width && self.height > contained.height
    }
}

fn main() {
    let rect1 = Rectangle {width:10, height: 69};
    let rect2 = Rectangle {width:4, height:18};

    println!("The rectangle's area is {}.", rect1.area());
    if rect1.can_contain(&rect2) { println!("It can contain rect2"); };

    let sq = Rectangle::square(10);
}
