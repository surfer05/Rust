let keyword is used to declare a variable
mut keyword is used to make a keyword mutable

println! calls a Rust macro. If it had called a function instead, it would be entered as println (without the !).For now, you just need to know that using a ! means that you’re calling a macro instead of a normal function and that macros don’t always follow the same rules as functions.

Rust is an ahead-of-time compiled language, meaning you can compile a program and give the executable to someone else, and they can run it even without having Rust installed. If you give someone a .rb, .py, or .js file, they need to have a Ruby, Python, or JavaScript implementation installed (respectively). But in those languages, you only need one command to compile and run your program. Everything is a trade-off in language design.

It has also initialized a new Git repository along with a .gitignore file. Git files won’t be generated if you run cargo new within an existing Git repository; you can override this behavior by using `cargo new --vcs=git`.

// For Cargo.toml
TOML - Tom's obvious, Minimal language

The first line, [package], is a section heading that indicates that the following statements are configuring a package. As we add more information to this file, we’ll add other sections.

The next three lines set the configuration information Cargo needs to compile your program: the name, the version, and the edition of Rust to use. 

The last line, [dependencies], is the start of a section for you to list any of your project’s dependencies. In Rust, packages of code are referred to as crates.
// end for Cargo.toml

Cargo also provides a command called cargo check. This command quickly checks your code to make sure it compiles but doesn’t produce an executable:

// For the guessing_game code Listing 2-1:

To obtain user input and then print the result as output, we need to bring the io input/output library into scope. The io library comes from the standard library, known as std:
use std::io;

String::new, a function that returns a new instance of a String. String is a string type provided by the standard library that is a growable, UTF-8 encoded bit of text.

The :: syntax in the ::new line indicates that new is an associated function of the String type. An associated function is a function that’s implemented on a type, in this case String. This new function creates a new, empty string. You’ll find a new function on many types because it’s a common name for a function that makes a new value of some kind.

The full job of read_line is to take whatever the user types into standard input and append that into a string (without overwriting its contents), so we therefore pass that string as an argument. The string argument needs to be mutable so the method can change the string’s content.

Like variables, references are immutable by default. Hence, you need to write &mut guess rather than &guess to make it mutable.

read_line puts whatever the user enters into the string we pass to it, but it also returns a Result value.

Result is an enumeration, often called an enum, which is a type that can be in one of multiple possible states. We call each possible state a variant. Result’s variants are Ok and Err. The Ok variant indicates the operation was successful, and inside Ok is the successfully generated value. The Err variant means the operation failed, and Err contains information about how or why the operation failed.

When printing the value of a variable, the variable name can go inside the curly brackets. When printing the result of evaluating an expression, place empty curly brackets in the format string, then follow the format string with a comma-separated list of expressions to print in each empty curly bracket placeholder in the same order. Printing a variable and the result of an expression in one call to println! would look like this:

let x = 5;
let y = 10;

println!("x = {x} and y + 2 = {}", y + 2);
This code would print x = 5 and y = 12

The Rng trait defines methods that random number generators implement, and this trait must be in scope for us to use those methods.

The Ordering type is another enum and has the variants Less, Greater, and Equal. These are the three outcomes that are possible when you compare two values.

The cmp method compares two values and can be called on anything that can be compared. It takes a reference to whatever you want to compare with: here it’s comparing guess to secret_number.

A match expression is made up of arms. An arm consists of a pattern to match against, and the code that should be run if the value given to match fits that arm’s pattern.

Shadowing lets us reuse the guess variable name rather than forcing us to create two unique variables, such as guess_str and guess, for example. We’ll cover this in more detail in Chapter 3, but for now, know that this feature is often used when you want to convert a value from one type to another type.

The trim method on a String instance will eliminate any whitespace at the beginning and end, which we must do to be able to compare the string to the u32, which can only contain numerical data.

The user must press enter to satisfy read_line and input their guess, which adds a newline character to the string. For example, if the user types 5 and presses enter, guess looks like this: 5\n. The \n represents “newline.” (On Windows, pressing enter results in a carriage return and a newline, \r\n.) The trim method eliminates \n or \r\n, resulting in just 5.

The parse method on strings converts a string to another type. Here, we use it to convert from a string to a number. We need to tell Rust the exact number type we want by using let guess: u32. The colon (:) after guess tells Rust we’ll annotate the variable’s type. Rust has a few built-in number types; the u32 seen here is an unsigned, 32-bit integer. It’s a good default choice for a small positive number.

parse returns a Result type and Result is an enum that has the variants Ok and Err. We’re using a match expression here, as we did with the Ordering result of the cmp method.

If parse is able to successfully turn the string into a number, it will return an Ok value that contains the resultant number. That Ok value will match the first arm’s pattern, and the match expression will just return the num value that parse produced and put inside the Ok value. That number will end up right where we want it in the new guess variable we’re creating.

The underscore, _, is a catchall value; in this example, we’re saying we want to match all Err values, no matter what information they have inside them. So the program will execute the second arm’s code, continue, which tells the program to go to the next iteration of the loop and ask for another guess. So, effectively, the program ignores all errors that parse might encounter!

// Chapter 3: Variables And Mutability
You declare constants using the const keyword instead of the let keyword, and the type of the value must be annotated.

Rust’s naming convention for constants is to use all uppercase with underscores between words.

Rustaceans say that the first variable is shadowed by the second, which means that the second variable is what the compiler will see when you use the name of the variable. In effect, the second variable overshadows the first, taking any uses of the variable name to itself until either it itself is shadowed or the scope ends.

Shadowing is different from marking a variable as mut because we’ll get a compile-time error if we accidentally try to reassign to this variable without using the let keyword. By using let, we can perform a few transformations on a value but have the variable be immutable after those transformations have been completed.

The other difference between mut and shadowing is that because we’re effectively creating a new variable when we use the let keyword again, we can change the type of the value but reuse the same name. For example, say our program asks a user to show how many spaces they want between some text by inputting space characters, and then we want to store that input as a number:

    let spaces = "   ";
    let spaces = spaces.len();

    The first spaces variable is a string type and the second spaces variable is a number type. Shadowing thus spares us from having to come up with different names, such as spaces_str and spaces_num; instead, we can reuse the simpler spaces name. However, if we try to use mut for this, as shown here, we’ll get a compile-time error:

    let mut spaces = "   ";
    spaces = spaces.len();
The error says we’re not allowed to mutate a variable’s type.

// Chapter 3 : Data Types
Two subsets : Scalar and Compound
Scalar -> integers,floating-point , booleans, characters
integers
8-bit	i8	u8
16-bit	i16	u16
32-bit	i32	u32
64-bit	i64	u64
128-bit	i128	u128
arch	isize	usize
Additionally, the isize and usize types depend on the architecture of the computer your program is running on, which is denoted in the table as “arch”: 64 bits if you’re on a 64-bit architecture and 32 bits if you’re on a 32-bit architecture.

Rust’s char type is four bytes in size and represents a Unicode Scalar Value, which means it can represent a lot more than just ASCII. Accented letters; Chinese, Japanese, and Korean characters; emoji; and zero-width spaces are all valid char values in Rust. Unicode Scalar Values range from U+0000 to U+D7FF and U+E000 to U+10FFFF inclusive. However, a “character” isn’t really a concept in Unicode, so your human intuition for what a “character” is may not match up with what a char is in Rust.

Compound -> tuples and arrays
tuples - fixed length
The tuple without any values has a special name, unit. This value and its corresponding type are both written () and represent an empty value or an empty return type. Expressions implicitly return the unit value if they don’t return any other value.

arrays - fixed length
every element of array must have the same type

Statements are instructions that perform some action and do not return a value.
Expressions evaluate to a resultant value.

// Chapter 4 : What is ownership?
A scope is the range within a program for which an item is valid. 
String literals are immutable. "string literals" are different from "String" data type

You can create a String from a string literal using the from function, like so:

let s = String::from("hello");

example - string x = "5"; // 5 is a string literal

There is a natural point at which we can return the memory our String needs to the allocator: when s goes out of scope. When a variable goes out of scope, Rust calls a special function for us. This function is called `drop`, and it’s where the author of String can put the code to return the memory. Rust calls drop automatically at the closing curly bracket.  

let x = 5;
let y = x;
We can probably guess what this is doing: “bind the value 5 to x; then make a copy of the value in x and bind it to y.” We now have two variables, x and y, and both equal 5. This is indeed what is happening, because integers are simple values with a known, fixed size, and these two 5 values are pushed onto the stack.

The length is how much memory, in bytes, the contents of the String are currently using. The capacity is the total amount of memory, in bytes, that the String has received from the allocator. 

If we do want to deeply copy the heap data of the String, not just the stack data, we can use a common method called clone. We’ll discuss method syntax in Chapter 5, but because methods are a common feature in many programming languages, you’ve probably seen them before.

Here’s an example of the clone method in action:

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
This works just fine.

integers that have a known size at compile time are stored entirely on the stack, so copies of the actual values are quick to make. That means there’s no reason we would want to prevent x from being valid after we create the variable y. In other words, there’s no difference between deep and shallow copying here, so calling clone wouldn’t do anything different from the usual shallow copying, and we can leave it out.

Here are some of the types that implement Copy:

All the integer types, such as u32.
The Boolean type, bool, with values true and false.
All the floating-point types, such as f64.
The character type, char.
Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, but (i32, String) does not.

Just as variables are immutable by default, so are references. We’re not allowed to modify something we have a reference to.
change(&s); // we can't change string in the function as the reference isn't mutable

but 
change(&mut s);
fn change(some_string: &mut String) {
    some_string.push_str(", world");
} 
this code will allow to change the borrowed reference as we are referencing it as mutable

Mutable references have one big restriction: if you have a mutable reference to a value, you can have no other references to that value. This code that attempts to create two mutable references to s will fail:

This code does not compile!
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{}, {}", r1, r2);

We can use immutable references more than once for separated variables but it is not possible for mutable references.
Also, mutable and immutable reference for the same data will not be possible. 
i.e. 
let mut s = String::from("hello");

let r1 = &s; // no problem
let r2 = &s; // no problem
let r3 = &mut s; // BIG PROBLEM


NOTABLE POINTS:
->At any given time, you can have either one mutable reference or any number of immutable references.
->References must always be valid.

// Slices 
Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection. A slice is a kind of reference, so it does not have ownership.

// Chapter 7 : Packages and Crates
Library crates don’t have a main function, and they don’t compile to an executable. Instead, they define functionality intended to be shared with multiple projects. For example, the rand crate we used in Chapter 2 provides functionality that generates random numbers. Most of the time when Rustaceans say “crate”, they mean library crate, and they use “crate” interchangeably with the general programming concept of a “library".

Cargo is actually a package that contains the binary crate for the command-line tool you’ve been using to build your code. The Cargo package also contains a library crate that the binary crate depends on. Other projects can depend on the Cargo library crate to use the same logic the Cargo command-line tool uses.

Cargo follows a convention that src/main.rs is the crate root of a binary crate with the same name as the package. Likewise, Cargo knows that if the package directory contains src/lib.rs, the package contains a library crate with the same name as the package, and src/lib.rs is its crate root. Cargo passes the crate root files to rustc to build the library or binary.

If a package contains src/main.rs and src/lib.rs, it has two crates: a binary and a library, both with the same name as the package. A package can have multiple binary crates by placing files in the src/bin directory: each file will be a separate binary crate.