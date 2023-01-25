// Basics

// fn main() {
//     let s1 = String::from("hello");

//     let len = calculate_length(&mut s1);

//     println!("The length of '{}' is {}.", s1, len);
// }

// fn calculate_length(s: &mut String) -> usize {

//     // produces error as the borrowed value is not mutable
//     // some_string.push_str(",world!");

//     // not it will work as now the reference is mutable
//     some_string.push_str(",world!");

// }

// Mutable references

// fn main() {
//     let mut s = String::from("Hello");

//     change(&mut s);
//     println!("{}",s);
// }


// fn change(some_string: &mut String) {
//     some_string.push_str(", world!");
// }

// Mutable and immutable references for the same data

fn main(){
    // Valid code 
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;

    // Invalid code
            // fn main() {
            //     let mut s = String::from("hello");
            
            //     let r1 = &s; // no problem
            //     let r2 = &s; // no problem
            //     let r3 = &mut s; // BIG PROBLEM
    
            //     println!("{}, {}, and {}", r1, r2, r3);
            // }   
            
    // Valid code
    fn main() {
        let mut s = String::from("hello");
    
        let r1 = &s; // no problem
        let r2 = &s; // no problem
        println!("{} and {}", r1, r2);
        // variables r1 and r2 will not be used after this point
    
        let r3 = &mut s; // no problem
        println!("{}", r3);
    }
    
}
// Whew! We also cannot have a mutable reference while we have an immutable one to the same value.