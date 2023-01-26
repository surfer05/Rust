#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {          //&self is short for self:&Self
        self.width * self.height
    }
    fn width(&self)-> bool{
        self.width >0
    }
    fn can_hold(&self , other: &Rectangle)->bool{
        self.width > other.width && self.height > other.height
    }

    // Associated Function
    // The Self keywords in the return type and in the body of the function are aliases for the type that appears after the impl keyword, which in this case is Rectangle.
    fn square(size:u32)-> Self{
        Self{
            width:size,
            height:size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels. ",
        rect1.area()
    );
    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {} ", rect1.width);
    }

    let rect2 = Rectangle {
        width:10,
        height:40,
    };
    let rect3 = Rectangle{
        width:60,
        height:45,
    };
    let sq = Rectangle::square(3);
    println!("{}",sq.area());

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

// Methods are similar to functions: we declare them with the fn keyword and a name, they can have parameters and a return value, and they contain some code that’s run when the method is called from somewhere else.

//  Within an impl block, the type Self is an alias for the type that the impl block is for. 


// Note that we can choose to give a method the same name as one of the struct’s fields. For example, we can define a method on Rectangle also named width

// In main, when we follow rect1.width with parentheses, Rust knows we mean the method width. When we don’t use parentheses, Rust knows we mean the field width.

// Associated functions that aren’t methods are often used for constructors that will return a new instance of the struct. 