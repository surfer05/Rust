// fn main() {
//     let width1 = 30;
//     let height1 = 50;

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(width1, height1)
//     );
// }

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// // Refactoring with Tuples

// fn main() {
//     let rect1 = (30, 50);

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(rect1)
//     );
// }

// fn area(dimensions:(u32,u32))->u32{
//     dimensions.0*dimensions.1
// }

// Refactoring with Structs


// #[derive(Debug)]
// struct Rectangle{
//     width:u32,
//     height:u32,
// }

// fn main(){
//     let rect1 = Rectangle{
//         width:30,
//         height:50,
//     };

//     println!( "rect is {:#?}",rect1);
// }


// The area function accesses the width and height fields of the Rectangle instance (note that accessing fields of a borrowed struct instance does not move the field values, which is why you often see borrows of structs).

fn area(rectangle: &Rectangle)-> u32 {
    rectangle.width * rectangle.height 
}

// Using dbg!

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
}