fn main(){
    let config_max = Some(3u8);

    // As we do through match
    match config_max {
        Some(max) => println!("")
        _ => (),
    }

    // Doing it through if let
    if let Some(max) = config_max {
        println!("The maximum is configured to be {} ", max);
    }
}

// Using match
let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }


// Using if let and else
let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }

// DON'T RUN THIS PROGRAM DIRECTLY CAUSE ALL CODE SNIPPETS HAVE BEEN THAT WERE REQUIRED FOR LEARNING, AND IF YOU WANT TO RUN IT,BETTER COMMENT OUT THE REST OF THE CODE THAT ISN'T NECESSARY. 