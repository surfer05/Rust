// Final function
fn main(){
    let my_string = String::from("hello world");
    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent 
    // to whole slices of `String`s
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
}







// fn main() {
//     let s = String::from("hello");

//     let slice = &s[0..2];
//     println!("{}", slice);
//     let slice = &s[..2];
//     println!("{}", slice);
    
//     let len = s.len();
    
//     let slice = &s[3..len];
//     println!("{}", slice);
    
//     let slice = &s[3..];
//     println!("{}", slice);
// }

// fn first_word(s: &String) -> &str {
//     let bytes = s.as_bytes();

//     for(i,&item) in bytes.iter().enumerate() {
//         if item == b' '{
//             return &s[0..i];
//         }
//     }
//     &s[..]
// }

// // fn first_word(s: &String) -> usize {
// //     // we’ll convert our String to an array of bytes using the as_bytes method.
// //     let bytes = s.as_bytes();

// //     // Next, we create an iterator over the array of bytes using the iter method:
// //     // iter is a method that returns each element in a collection and that enumerate wraps the result of iter and returns each element as part of a tuple instead.
// //     for (i, &item) in bytes.iter().enumerate() {
// //         if item == b' ' {
// //             return i;
// //         }
// //     }

// //     s.len()
// // }

// // The first element of the tuple returned from enumerate is the index, and the second element is a reference to the element. This is a bit more convenient than calculating the index ourselves.

// // s.clear(); // this empties the String, making it equal to ""
