#[derive(Debug)] // derive macro
struct Rectangle {
    width: u32,
    height: u32,
} // struct

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    } // method

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    } // method
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    } // associated function
}

fn main() {
    struct _Color(i32, i32, i32); // tuple struct
    struct _Point(i32, i32, i32); // tuple struct

    let rect = Rectangle {
        width: 30,
        height: 50,
    }; // instance

    println!("rect is {:#?}", rect); // rect is Rectangle {
                                        //   width: 30,
                                        //   height: 50,
                                        // } 

    // println!("The area of the rectangle is {} square pixels.", Rectangle::area(&rect));  // This also works
    println!("The area of the rectangle is {} square pixels.", rect.area()); // The area of the rectangle is 1500 square pixels.

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    }; // instance
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    }; // instance
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    }; // instance
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2)); // Can rect1 hold rect2? true
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3)); // Can rect1 hold rect3? false

    let sq = Rectangle::square(3);  
    println!("sq is {:#?}", sq); // sq is Rectangle {
                                 //   width: 3,
                                 //   height: 3,
                                 // }
}