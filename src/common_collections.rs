// The first type of collection we'll look at is Vec<T>
// aka a vector
// Vectors can allow you to store more than one value
// in a single data structure that puts all the values next to eachother in memory
// Their size is not fixed

pub fn vectors() {
    // The first way to create a vector is with an empty vector
    let mut v: Vec<i32> = Vec::new();

    // then you can push elements with the push method
    v.push(5);
    v.push(11);
    
    // if you want to infer start values, you can use the vec! macro
    let vm = vec![1, 2, 3];
    // this will automatically infer type of i32

    // There are two ways to get indexes from vectors
    // by using the classic array indexing
    // or by using the get() method, which returns Option<&T>

    // if we pass in any number > 2, this will print "doesn't exist"
    match vm.get(2) {
        None => println!("doesn't exist"),
        _ => println!("&vm[2] = {}", &vm[2])
    }

    

    // Iterating over values in a vector
    // add 50 to each element
    let mut v3 = vec![100, 927, 6];
    for i in &mut v3 {
        // * is the dereference operator, and yes, it's confusing coming from C
        *i += 50;
        println!("i + 50 = {}", i);
    }

    // to store multiple values in a vector, you can use an enum
    // I won't go into an example here, since you can probably imagine how this would work

} // <- v and vm go out of scope are are freed

pub fn strings() {
    // There is two main types of string
    // Rust has only one type in core language, which is string slice str
    // it's usually in the borrowed form &str
    // the String type is in rust's standard library
    // It's growable (?), mutable, and an owned

    // String is similar to ectors in a way
    let mut stdstring = String::new();

    // you can use the .to_string() method for converting string literals to the type
    let mut more_string = "contents".to_string();

    // String::from does the exact same thing
    // and the compiler will recognize it as the same
    // so which one you use is a matter of style
    // Also strings are utf8 encoded
    // so emojis and any properly encoded data can be used

    // adding to a string
    more_string.push_str(" again");
    stdstring.push_str("what in tarnation");

    // oh and a string is a wrapper over Vec<u8>
    // some values don't make sense to be indexed
    // as they are diacritic characters
    // these are used in many languages, such as hindi and japanese
    // the compiler knows this and will properly throw an error when you try to
    // index a diacritic character

    // here are some methods
    
    // chars seperates a string into individual unicode scalars
    for c in more_string.chars() {
        println!("{}", c);
    }

    // bytes will return each raw byte
    for b in stdstring.bytes() {
        println!("{}", b);
    }

    // Strings are complicated. Used with care
}

// hash maps sound complicated but they are just dictionaries
// or maps
// or tables
// whatever you want to call it
// they are used with 2 generics HashMap<K, V>
// k and v are key and value

use std::collections::HashMap;

pub fn hash_maps() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 50);

    // here is a weird one
    let teams = vec!["blue".to_string(), "red".to_string()];
    let scores_initial = vec![10, 50];

    let scoremap: HashMap<_, _> = teams.iter().zip(scores_initial.iter()).collect();
    // the type HashMap<_, _> is needed to collect into many different data strutures
    // and rust doesn't know which one you want
    // underscores mean it will infer type based on data

    for(key, value) in &scoremap {
        println!("{}: {}", key, value);
    }
}