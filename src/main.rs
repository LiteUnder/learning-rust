fn main() {
    println!("Hello, world!");
    println!("{0}", add(7, 9)) // last line of scope doesn't need a semi colon, interesting
}

// A few interesting notes on syntax here
// paramaters are declared as <name>: <type>
// return type isn't required, but syntax is -> <type>
fn add(a: i32, b: i32) -> i32 {
    a + b // last line of function is always run as a return
}