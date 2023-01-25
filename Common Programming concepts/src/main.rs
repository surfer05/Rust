use std::io;
fn main() {
    // <-------------Variables and mutability---------------->


    // let x = 5;

    // let x = x + 1;

    // {
    //     let x = x * 2;
    //     println!("The value of x in the inner scope is: {x}");
    // }
    // println!("The value of x is:{x} ");


    // <---------------------Data Types-------------------->
    // Tuples

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let z = tup.2;
    let x = tup.0;
    let y = tup.1;

    println!("The required values are :{x},{y},{z}");

    // Arrays
    
    let _b: [i32; 5] = [1, 2, 3, 4, 5];
    let _c = [3; 5];

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index:usize = index.trim().parse().expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element} ");
}
