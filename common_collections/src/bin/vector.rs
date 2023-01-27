fn main(){
    println!("Hello world!");

    // To create a new empty vector, we call the Vec::new function
    let v: Vec<i32> = Vec::new();

    // Rust conveniently provides the vec! macro, which will create a new vector that holds the values you give it.
    let v2 = vec![1,2,3];  //i32 is the defaoult data type for integers

    // To create a vector and then add elements to it, we can use the push method
    let mut v3 = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // There are two ways to reference a value stored in a vector: via indexing or using the get method.
    let v = vec![1,2,3,4,5];

    // Using & and [] gives us a reference to the element at the index value. 
    let third: &i32 = &v[2];          
    println!("The third element is {third}");

    // When we use the get method with the index passed as an argument, we get an 
    // Option<&T> that we can use with match
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    // The reason Rust provides these two ways to reference an element is so you can choose how the program behaves when you try to use an index value outside the range of existing elements. As an example, let’s see what happens when we have a vector of five elements and then we try to access an element at index 100 with each technique,

    let v = vec![1,2,3,4,5];
    // the [] method will cause the program to panic because it references a nonexistent element. This method is best used when you want your program to crash if there’s an attempt to access an element past the end of the vector.
    let does_not_exist_1 = &v[100];

    // When the get method is passed an index that is outside the vector, it returns None without panicking. You would use this method if accessing an element beyond the range of the vector may happen occasionally under normal circumstances. Your code will then have logic to handle having either Some(&element) or None.
    let does_not_exist_2 = v.get(100);

    //When the program has a valid reference, the borrow checker enforces the ownership and borrowing rulest to ensure this reference and any other references to the contents of the vector remain valid.

    // the rule that states you can’t have mutable and immutable references in the same scope.
    // The code below won't work
    let mut v = vec![1,2,3,4,5];
    let first = &v[0];  //immutable borrow occurs here
    v.push(6);          //mutable borrow occurs here
    println!("The first element is: {first}");
    // This error is due to the way vectors work: because vectors put the values next to each other in memory, adding a new element onto the end of the vector might require allocating new memory and copying the old elements to the new space, if there isn’t enough room to put all the elements next to each other where the vector is currently stored. In that case, the reference to the first element would be pointing to deallocated memory.

    // ITERATING OVER THE VALUES IN THE VECTOR

    // use a for loop to get immutable references to each element in a vector of i32 values and print them.
    let v4 = vec![100,32,57];
    for i in &v {
        println!("{i}");
    }

    // We can also iterate over mutable references to each element in a mutable vector in order to make changes to all the elements. 
    let mut v5 = vec![100,32,57];
    for i in &mut v{
        *i += 50;      
        //To change the value that the mutable reference refers to, we have to use the * dereference operator to get to the value in i before we can use the += operator. 
    }

    // USING AN ENUM TO STORE MULTIPLE TYPES
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ]
}