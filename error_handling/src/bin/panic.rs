fn main(){
    let v = vec![1,2,3];
    // Using [] is supposed to return an element, but if you pass an invalid index, there’s no element that Rust could return here that would be correct.
    v[99];

    // After error location, next note tells us that set the RUST_BACKTRACE=1  to display a backtrace.

    //  A backtrace is a list of all the functions that have been called to get to this point. Backtraces in Rust work as they do in other languages: the key to reading the backtrace is to start from the top and read until you see files you wrote.

}

// BUFFER OVERREAD
// You might get whatever is at the location in memory that would correspond to that element in the data structure, even though the memory doesn’t belong to that structure. This is called a buffer overread and can lead to security vulnerabilities if an attacker is able to manipulate the index in such a way as to read data they shouldn’t be allowed to that is stored after the data structure.