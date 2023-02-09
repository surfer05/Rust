// // #[cfg(test)]
// // mod tests {
// //     #[test]
// //     fn add_two() {
// //         assert_eq!(2 + 2, 4);
// //     }
// //     #[test]
// //     fn failed() {
// //         panic!("This test failed!")
// //     }
// // }

// #[warn(dead_code)]
// #[derive(Debug)]
// struct Rectangle {
//     length: u32,
//     width: u32,
// }

// impl Rectangle {
//     fn can_hold(&self, other: &Rectangle) -> bool {
//         self.width > other.width && self.length > other.length
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn larger_holds_smaller() {
//         let larger = Rectangle {
//             length: 8,
//             width: 7,
//         };
//         let smaller = Rectangle {
//             length: 5,
//             width: 1,
//         };
//         assert!(larger.can_hold(&smaller));
//     }
// }

// pub fn add_two(a: i32) -> i32 {
//     return a + 5;
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn check_addition() {
//         assert_eq!(add_two(4), 6)
//     }
// }

// pub fn greeting(greet: &str) -> String {
//     String::from("Hello")
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn name_exists() {
//         let result = greeting("Carol");
//         assert!(
//             result.contains("Carol"),
//             "Greeeting did not contain name, the result was `{}`",
//             result
//         );
//     }
// }

// pub struct Guess {
//     value: i32,
// }

// impl Guess {
//     pub fn new(value: i32) -> Guess {
//         if value < 1 {
//             panic!(
//                 "Guess value must be less than or equal to 100, got {}.",
//                 value
//             );
//         } else if value > 100 {
//             panic!(
//                 "Guess value must be greater than or equal to 1, got {}.",
//                 value
//             );
//         }

//         Guess { value }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     #[should_panic(expected = "less than or equal to 100")]
//     fn greater_than_100() {
//         Guess::new(200);
//     }
// }

// #[cfg(test)]
// mod tests {
//     fn add_two() -> Result<(), String> {
//         if 2 + 2 == 4 {
//             Ok(())
//         } else {
//             Err(String::from("Not Equal!"))
//         }
//     }
// }

// fn prints_and_returns_10(a: i32) -> i32 {
//     println!("I got the value {}", a);
//     10
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn this_test_will_pass() {
//         let value = prints_and_returns_10(4);
//         assert_eq!(10, value);
//     }

//     #[test]
//     fn this_test_will_fail() {
//         let value = prints_and_returns_10(8);
//         assert_eq!(5, value);
//     }
// }

// pub fn add_two(a: i32) -> i32 {
//     a + 2
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn check_with_3() {
//         assert_eq!(5, add_two(3));
//     }

//     #[test]
//     fn check_with_2() {
//         assert_eq!(4, add_two(2));
//     }

//     #[test]
//     fn check_with_98() {
//         assert_eq!(100, add_two(98));
//     }
// }

pub fn add_two(a:i32)->i32{
    internal_addition(a,2)
}

fn internal_addition(a:i32,b:i32)->i32{
    a+b
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn internal(){
        assert_eq!(4,internal_addition(2,2));
    }
}