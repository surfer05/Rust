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
    let greeting_file = File::open("hello.txt").unwrap();

    // expect : the `expect` method lets us also choose the panic! error message. Using expect instead of unwrap and providing good error messages can convey your intent and make tracking down the source of a panic easier. 

    // expect in action
    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be incuded in this project.");

        
    }
    
// PROPAGATING ERRORS

fn read_username_from_file()-> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result{
        Ok(file)=>file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();
    match username_file.read_to_string(&mut username){
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }


    // A Shortcut for Propagating Errors: the ? Operator
    fn read_username_from_file() -> Result<String, io::Error> {
        let mut username_file = File::open("hello.txt")?;
        let mut username = String::new();
        // Value of result is Ok : value inside Ok is returned
        // If the value is Err : Err will be returned from whole function
        username_file.read_to_string(&mut username)?;
        Ok(username)
    }
    // We’re only allowed to use the ? operator in a function that returns Result, Option, or another type that implements FromResidual.

    // USE fs::read_to_string instead of opening and the reading the file
    fn read_username_from_file()->{
        Result<String,io::Error>{
            // fs::read_to_string function that opens the file, creates a new String, reads the contents of the file, puts the contents into that String, and returns it.
            fs::read_to_string("hello.txt")
        }
    }

    // USING the ? operator on an Option<T> value

    // This function returns Option<char> because it’s possible that there is a character there, but it’s also possible that there isn’t. 
    // This code takes the text string slice argument and calls the lines method on it, which returns an iterator over the lines in the string.
    // Because this function wants to examine the first line, it calls next on the iterator to get the first value from the iterator.
    // If text is the empty string, this call to next will return None, in which case we use ? to stop and return None from last_char_of_first_line.
    // If text is not the empty string, next will return a Some value containing a string slice of the first line in text.
    // The ? extracts the string slice, and we can call chars on that string slice to get an iterator of its characters.
    // We’re interested in the last character in this first line, so we call last to return the last item in the iterator
    fn last_char_of_first_line(text: &str)->Option<char> {
        text.lines().next()?.chars().last()
    }


    // RETURNING ERROR THROUGH MAIN FUNCTION
    // For now Box<dyn Error> means "any kind of error"
    // Even though the body of this main function will only ever return errors of type std::io::Error, by specifying Box<dyn Error>, this signature will continue to be correct even if more code that returns other errors is added to the body of main.


    fn main()->Result<(),Box<dyn Error>> {
        let greeting_file = File::open("hello.txt")?;

        Ok(())
    }


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