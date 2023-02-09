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

pub fn greeting(greet: &str) -> String {
    String::from("Hello")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn name_exists() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeeting did not contain name, the result was `{}`",
            result
        );
    }
}
