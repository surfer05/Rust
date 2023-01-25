use std::io;

fn main(){
    println!("Hello World!");
    another_function(23);
    print_labeled_measurement(5,'h');
    let a = five();
    println!("the value returned from function is :{a} ");
    let b = plus_one(10);
    println!("The value of x is : {b} ");
}

fn another_function(x : i32){
    println!("The value of x is:{x} ");
}

fn print_labeled_measurement(value:i32,unit_label:char){
    println!("The measurement is : {value}{unit_label} ");
}

fn five() -> i32{
    5
}

fn plus_one(x:i32) ->i32 {
    x+1
}
