fn main() {
    println!("Hello, world!");

    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    // user1.email = String::from("anotheremail@example.com");

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    let user3 = User {
        email : String::from("another@example.com"),
        ..user2
    };

    // Tuple Structs
    struct Color(i32,i32,i32);
    struct Point(i32,i32,i32);

    fn main(){
        let black = Color(0,0,0);
        let origin = Point(0,0,0);
    }

    // Unit-Like Structs
    struct AlwaysEqual;
    let subject = AlwaysEqual;
}

// fn build_user(email:String,username:String)-> User{


//     // User{
//     //     email:email,
//     //     username:username,
//     //     active:true,
//     //     sign_in_count:1,
//     // };
//     // with field init
//     User{
//         email,               //instead of email:email
//         username,           // instead of username:username
//         active:true,
//         sign_in_count:1,
//     }
// }

// Note that the entire instance must be mutable; Rust doesn’t allow us to mark only certain fields as mutable. As with any expression, we can construct a new instance of the struct as the last expression in the function body to implicitly return that new instance.

// Using struct update syntax, we can achieve the same effect with less code, as shown in Listing 5-7. The syntax .. specifies that the remaining fields not explicitly set should have the same value as the fields in the given instance.


// Note that the struct update syntax uses `=` like an assignment; this is because it moves the data, just as we saw in the “Ways Variables and Data Interact: Move” section. In this example, we can no longer use user1 after creating user2 because the String in the username field of user1 was moved into user2. If we had given user2 new String values for both email and username, and thus only used the active and sign_in_count values from user1, then user1 would still be valid after creating user2. The types of active and sign_in_count are types that implement the Copy trait, so the behavior we discussed in the “Stack-Only Data: Copy” section would apply.