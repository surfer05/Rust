#[derive(Debug)]
struct Rectangle{
    width:u32,
    height:u32,
}

fn main(){
    let rect1 = Rectangle{
        width:30,
        height:50,
    };

    // gives error : `Rectangle` cannot be formatted with the default formatter
    println!("rect1 is {:#?}",rect1);

    // Another way to print out a value using the Debug format is to use the dbg! macro, which takes ownership of an expression (as opposed to println! that takes a reference)

    // dbg! prints the file and line number of where that dbg! macro call occurs in your code along with the resulting value of that expression, and returns ownership of the value.

    // We can put dbg! around the expression 30 * scale and, because dbg! returns ownership of the expression’s value, the width field will get the same value as if we didn’t have the dbg! call there. We don’t want dbg! to take ownership of rect1, so we use a reference to rect1 in the next call. 

    let scale =2;
    let rect1 = Rectangle{
        width:dbg!(30*1),
        height:50,
    };
    dbg!(&rect1);

    
}

// Rust does include functionality to print out debugging information, but we have to explicitly opt in to make that functionality available for our struct. To do that, we add the outer attribute #[derive(Debug)] just before the struct definition