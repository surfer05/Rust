// For finding largest number in two separate lists, we are writing the fuction again
            // fn main() {
            //     let number_list = vec![34, 50, 25, 100, 65];
            
            //     let mut largest = &number_list[0];
            
            //     for number in &number_list {
            //         if number > largest {
            //             largest = number;
            //         }
            //     }
            
            //     println!("The largest number is {}", largest);
            
            //     let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
            
            //     let mut largest = &number_list[0];
            
            //     for number in &number_list {
            //         if number > largest {
            //             largest = number;
            //         }
            //     }

            //     println!("The largest number is {}", largest);
            // }

// BETTER APPROACH

fn largest(list: &[i32])-> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item>largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}",result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest(&number_list);
    println!("The largest number is {}",result);

}

// HOW IS GENERICS USEFUL
// One that finds the largest item in a slice of i32 values and one that finds the largest item in a slice of char values. How would we eliminate that duplication?