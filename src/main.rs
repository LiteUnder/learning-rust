mod rects;

// A few interesting notes on syntax here
// paramaters are declared as <name>: <type>
// return type isn't required, but syntax is -> <type>
fn add(a: i32, b: i32) -> i32 {
    a + b // last line of function is always run as a return
}

// Lets break some shit, shall we?
// How a bout a dangling pointer
///fn dangle() -> &String {
//    let s = String::from("dangled"); // new string instance

//    &s // return references to string
//} // s goes out of scope, compile error

fn dont_dangle() -> String {
    String::from("ez")
}

// slices!!
// A slice is a data type that doesn't have ownership
// Reference sequence of elements in a collection rather than the whole collection
// &str would allow a string literal and String
fn first_word(s: &String) -> usize {
    // convert to array of bytes
    let bytes = s.as_bytes();

    // this is a little weird
    // iter() returns each element of a collection and enumerate() returns each elements as a part of a tuple
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            // b' ' gives the bytecode for a space
            return i;
        }
    }

    s.len() // not tied to state of variable after reference
}

// let's rewrite this with slices
// &str refers to a string slice
// also &String can just be &str, they are the same thing
fn first_word_better(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..] // &s != &s[..], once is a String reference and the other is a string slice reference
}

fn main() {
    // when function has ! at the end, it's a macro
    // idk the difference yet
    println!("Hello, world!");
    println!("{}", dont_dangle());

    // println! macro only takes string literal
    // {} is used for formatting
    // "{0} {1}", 12, 13 will evalulate as "12 13"
    // numbered values start from 0, nice
    println!("{}", add(7, 9));

    // you can open a new scope anywhere with {}
    {
        let mut s = String::from("testing stuff");
        let word = first_word(&s); // gets the value 7

        s.clear(); // empties the string, s is now ""

        println!("word is {} characters... or is it?", word);
    }

    // word is still 7, but that's useless now that there
    // is meaninglyfully useful to use with the value 7. word is now invalid
    // The solution is of course, string slices
    // A string slice references a part of a String

    {
        let ss = String::from("hmmm");

        let hm = &ss[0..2]; // not including end

        // this is similar to referencing the whole string, but with the [0..2] part
        // if you want to include the end, you can use ..=
        // e.g. &ss[0..=1] will also equal "hm"

        let hm2 = &ss[0..=1]; // including end
        let hm3 = &ss[0..=2]; // hmm

        assert_eq!(hm, hm2);

        println!("hm3: {}", hm3);
        // assert_eq!(hm, hm3);
        // panics, "hm" != "hmm"

        // when it starts from 0, you can drop the first value
        // these are equal
        let slice = &ss[0..2];
        println!("{}", slice);
        let slice = &ss[..2];
        println!("{}", slice);

        // by the same token, if a slice has the last byte of the string you can drop the tailing number

        let len = ss.len();

        // these are the same
        let slice = &ss[1..len];
        println!("{}", slice);
        let slice = &ss[1..];
        println!("{}", slice);
    } // slice, ss, hm3, hm2, & hm all go out of scope and are dereferenced

    // Better method
    {
        // no point in making this mutable since it cannot be mutably referenced after first_word_better()
        let mut s = String::from("yeah so that happened");

        let word = first_word_better(&s);

        // s.clear();
        // ^ throws error, not valid reference
        // you cannot reference something as mutable if it has a immutable reference

        println!("the first word is \"{}\"", word);
    }

    {
        // Next up on our list is string literals, how fun
        // a string literal is LITERALLY (lol) just a slice of a string
        
        let a_string = String::from("sure"); // type is String

        // string literals are immutable, because &str is an immutable reference
        let a_string_literal = "sure"; // type here is &str
        let a_string_slice = &a_string[..];

        assert_eq!(a_string_literal, a_string_slice); // both are immutable references (&str)
    }

    {
        // Struct stuff
        let bigrect = rects::build_rect(882, 854);

        println!("{} square crabs", bigrect.area());
    }
    // last line of scope doesn't need a semicolon, interesting
}
