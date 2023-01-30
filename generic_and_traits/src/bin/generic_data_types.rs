// fn main(){

// }

// GENERICS IN FUNCTION DEFINITIONS

// WRITING AS A NOOB

// fn largest_i32(list: &[i32])->&i32{
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// fn largest_char(list: &[char]) -> &char {
//     let mut largest = &list[0]

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// fn main() {
//     let number_list = vec![34, 50, 25, 100, 65];

//     let result = largest_i32(&number_list);
//     println!("The largest number is {}",result);

//     let char_list = vec!['y', 'm', 'a', 'q'];

//     let result = largest_char(&char_list);
//     println!("The largest char is {}",result);
// }

// NOW WRITING GENERICS

// The help text mentions std::cmp::PartialOrd, which is a trait, and we’re going to talk about traits in the next section. For now, know that this error states that the body of largest won’t work for all possible types that T could be. Because we want to compare values of type T in the body, we can only use types whose values can be ordered. To enable comparisons, the standard library has the std::cmp::PartialOrd trait that you can implement on types. By following the help text's suggestion, we restrict the types valid for T to only those that implement PartialOrd and this example will compile, because the standard library implements PartialOrd on both i32 and char.
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

// GENERICS IN STRUCT DEFINITIONS
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
}

// Note that because we’ve used only one generic type to define Point<T>, this definition says that the Point<T> struct is generic over some type T, and the fields x and y are both that same type, whatever that type may be. If we create an instance of a Point<T> that has values of different types, our code won’t compile.

//  struct Point<T> {
//      x: T,
//      y: T,
//  }
//
//  fn main() {
//      let wont_work = Point { x: 5, y: 4.0 };
//  }

// To define a Point struct where x and y are both generics but could have different types, we can use multiple generic type parameters. For example, we change the definition of Point to be generic over types T and U where x is of type T and y is of type U.

struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 1.0 };
}

// GENERICS IN ENUM DEFINITIONS

fn main() {
    enum Option<T> {
        Some(T),
        None,
    }

    // Enums can use multiple generic types as well.

    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
}

// GENERICS IN METHOD DEFINITIONS

struct Point<T> {
    x: T,
    y: T,
}

// By declaring T as a generic type after impl, Rust can identify that the type in the angle brackets in Point is a generic type rather than a concrete type.

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// We can also specify constraints on generic types when defining methods on the type. We could, for example, implement methods only on Point<f32> instances rather than on Point<T> instances with any generic type. In code below we use the concrete type f32, meaning we don’t declare any types after impl.

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
// The code above means the type Point<f32> will have a distance_from_origin method; other instances of Point<T> where T is not of type f32 will not have this method defined.

fn main() {
    let p = Point { x: 5, y: 10 };
    println!("p.x = {} ", p.x());
}

// MIXED GENERICS
struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1mY2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}
fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {} , p3.y = {}", p3.x, p3.y);
}
// The purpose of this example above is to demonstrate a situation in which some generic parameters are declared with impl and some are declared with the method definition. Here, the generic parameters X1 and Y1 are declared after impl because they go with the struct definition. The generic parameters X2 and Y2 are declared after fn mixup, because they’re only relevant to the method.

// NOTE: THE USAGE OF GENERICS DOESN'T MAKE OUR CODE RUN ANY SLOWER
// REASON BELOW

// Rust accomplishes this by performing monomorphization of the code using generics at compile time. Monomorphization is the process of turning generic code into specific code by filling in the concrete types that are used when compiled.

// When Rust compiles this code, it performs monomorphization. During that process, the compiler reads the values that have been used in Option<T> instances and identifies two kinds of Option<T>: one is i32 and the other is f64. As such, it expands the generic definition of Option<T> into two definitions specialized to i32 and f64, thereby replacing the generic definition with the specific ones.

// MONOMORPHIZED CODE LOOKS SOMEWHAT SIMILAR TO THE CODE BELOW

enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}

fn main(){
    let integer = Option_i32(5);
    let float = Option(5.0);
}