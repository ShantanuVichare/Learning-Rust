// Rectangles is an example to using and implementing Structs


// Deriving Debug Trait for printing debug output for Rectangle struct
#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,
}

pub fn run(){
    let rect1 = Rectangle{ width: 30, height: 60 };

    // Elaborated debug output is shown by {:#?}
    println!("rect1 is a {:#?}", rect1);
    println!("Area of rect1: {}", area_func(&rect1));

    let rect2 = Rectangle{ height: 40, ..rect1 };
    println!("Area of rect2: {}", rect2.area());

    println!("Can rect1 hold rect2 ? {}", rect1.can_hold(&rect2));

    rect1.drop();
    // rect1 is invalidated since it was moved into .drop() method and not returned


    // Associated functions let you namespace functionality particular to you struct with instantiation
    println!("A square: {:?}", Rectangle::square(20));

}

// Function to compute area of a Rectangle instance
fn area_func(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

// Implementation Block
impl Rectangle{

    // First parameter of a method is always self (take ownership/borrow/borrow mutably)
    // Here, we have borrowed self immutably
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Method with multiple parameters
    fn can_hold(&self, other_rect: &Rectangle) -> bool {
        (self.height >= other_rect.height) && (self.width >= other_rect.width)
    }

    // First parameter of a method can move the object in by taking ownership
    // This method takes the ownership of the object and doesn't even return it, hence the value is Droppped
    fn drop(self) {}

}

// Each struct can have multiple Implementation Blocks
impl Rectangle{

    // Associated functions are similar to Static methods in other langauages 
    // These do not take self as a parameter
    fn square(size: u32) -> Rectangle {
        Rectangle{height: size, width: size}
    }
}