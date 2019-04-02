// tuples are quite interesting

pub fn tuples() {
    // Tuples are a fixed size after definition
    // each tuple element has a type
    // this one has i8, f64, & char
    // tuples allocate the heap
    let tup = (7i8, 2f64, 'a');

    // a tuple can be accessed via index (it's like an array sorta) like so
    let x = tup.0;
    println!("{}", x);

    // you can also "dereference" a tuple, like so

    let (x, y, z) = tup;
    println!("{}, {}, {}", x, y, z);

    // arrays are a thing in rust
    // they allocate to the stack as a single chunk of memory
    // fixed type & size
    // arrays are pretty similar to most other languages, other than the fixed size

    let array = [1, 2, 3, 4, 5];
    println!("{}", array[2]);
}