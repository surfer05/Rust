use std::fs::File;
fn main() {
    //RECOVERABLE ERRORS WITH RESULT
    let greeting_file_result = File::open("hello.txt");

    let _greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file : {:?} ", error),
    };

    // MATCHING ON DIFFERENT ERRORS

    // File::open returns inside the Err variant is io::Error ; this is struct provided by the standard library
    // This struct has a method `kind` that we can call to get an `io::ErrorKind` value
    // The enum io::ErrorKind is provided by the standard library and has variants representing the different kinds of errors that might result from an io operation.
    let greeting_file_result_2 = File::open("hello.txt");

    let _greeting_file_2 = match greeting_file_result_2 {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file:{:?}", e),
            },
            other_error => {
                panic!("Problem opening the file:{:?}", other_error);
            }
        },
    };
    // NOTE: We are using match statements very extensively and it definitely is exhausting. So we will be introducing the closures later to simplify the code.

    // However, The Result<T, E> type has many helper methods defined on it to do various, more specific tasks.
    // unwrap : Result is Ok variant, unwrap will return the value inside the Ok
    //           Result is the Err variant, unwrap will call the panic!

    // unwrap in action
    let greeting
}


// RESULT ENUM HAS TWO VARIANTS

// enum Result<T,E>{
//     Ok(T),
//     Err(E),
// }

// T represents the type of the value that will be returned in a success case within the Ok.
// E represents the type of the error that will be returned in a failure case within the Err variant.

// NOTE: open keyword opens a file


// Just a glimpse of how we can do the same code with closures


use std::io::File;
use std::io::ErrorKind;

fn main() {
    let _greeting_file = File::open("hello.txt").unwrap_or_else(|error|){
        if error.kind() == ErrorKind::NotFound{
            File::create("hello.txt").unwrap_or_else(|error|{
                panic!("Problem creating the file:{:?}",error);
            })
        } else {
            panic!("Problem opening the file:{:?}",error);
        }
    }
}