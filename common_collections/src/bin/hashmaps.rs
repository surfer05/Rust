use std::collections::HashMap;
fn main(){
    // The type HashMap<K, V> stores a mapping of keys of type K to values of type V using a hashing function, which determines how it places these keys and values into memory. 

    // CREATING A NEW HASH-MAP
    let must scores = HashMap::new();

    scores.insert(String::from("Blue"),10);
    scores.insert(String::from("Yellow"),50);

    // ACCESSING VALUES IN A HASH-MAP
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    // pub fn copied(self) -> Option<T>
    // where
    // T: Copy,
    // pub fn unwrap_or(self, default: T) -> T

    // The get method returns an Option<&V>; if there’s no value for that key in the hash map, get will return None. 
    //  This program handles the Option by calling copied to get an Option<i32> rather than an Option<&i32>
    // then unwrap_or to set score to zero if scores doesn't have an entry for the key.

    // We can iterate over each key/value pair in a hash map in a similar manner as we do with vectors, using a for loop

    for (key,value) in &scores {
        println!("{key}: {value} ");
    }

    // HASHMAPS AND OWNERSHIP

    // For types that implement the Copy trait, like i32, the values are copied into the hash map. For owned values like String, the values will be moved and the hash map will be the owner of those values


    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name,field_value);

    // We aren’t able to use the variables field_name and field_value after they’ve been moved into the hash map with the call to insert.

    // If we insert references to values into the hash map, the values won’t be moved into the hash map. The values that the references point to must be valid for at least as long as the hash map is valid. 

    // UPDATING A HASHMAP

    // Overwriting a Value
    // If we insert a key and a value into a hash map and then insert that same key with a different value, the value associated with that key will be replaced.
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    // Now Blue is associated with 25 and not 10

    // Adding a Key and Value Only If a Key Isn’t Present
    // Hash maps have a special API for this called `entry` that takes the key you want to check as a parameter.
    //The return value of the entry method is `an enum called Entry that represents a value that might or might not exist`.

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    // The first call to entry will insert the key for the Yellow team with the value 50 because the Yellow team doesn’t have a value already. The second call to entry will not change the hash map because the Blue team already has the value 10.

    // UPDATING A VALUE BASED ON THE OLD VALUE
    let text = "hello world , wonderful world";
    let mut mapn = HashMap::new();
    for word in text.split_whitespace(){
        // The `or_insert` method returns a mutable reference (&mut V) to the value for the specified key. Here we store that mutable reference in the count variable
        let count = mapn.entry(word).or_insert(0);
        //so in order to assign to that value, we must first dereference count using the asterisk (*)
        *count +=1;
    }

    

    
}