fn main() {
    println!("Hello, world!");
    let v = vec![1,2,3];
    // Using [] is supposed to return an element, but if you pass an invalid index, there’s no element that Rust could return here that would be correct.
    v[99];
}

