// so enums are also pretty interesting, I've never used them before
// an enum can be more appropriate than a struct sometimes
// it can "enumerate" across all the possible values
// to try and match a pattern

// we'll use this example from the book
// each variant of an enum can have types and amounts of data
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn enum_stuff() {
    let home = IPAddrKind::V4(127, 0, 0, 1);
    let loopback = IpAddrKind::V6(String::from("::1"));

    // message
    let m = Message::Write(String::from("Info"));
    m.call();
}

// a function can now take a type of ip address using the enum
fn route(ip_type: IpAddrKind) { }

// example of wide variety types
enum Message {
    Quit, // no data association
    Move {x: i32, y: i32}, // anonymous struct
    Write(String), // Single string
    ChangeColor(u8, u8, u8) // Three u8 values
}

// emums can have defined methods too

impl Message {
    fn call(&self) {
        println!("{}", self.Write)
    }
}
// cool stuff

// The Option<T> Enum, and how it's better than null
// null doesn't technically exist in rust
// so if a value at some point needs to have nothing
// all cases must be explicitly handled to convert from option
// to the type you want, otherwise the compiler will fail

fn option() {
    // Some(T) & None are variants of Option<T>
    let some_number = Some(5);
    let some_string = Some("a string");

    // Values that have None must have a type that
    // they should be when they are not None
    // for Some it doesn't matter, as the type is
    // specified by the value given
    let absent_number: Option<i32> = None;
    
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // even though y has a Some value, the case for when it
    // is None is not handled, so this won't work
    // let sum = x + y;
    // essentially, you can't add an i8 & a Option<i8> together
    
    // but, you can use match to handle these cases
    // match is exhaustive, meaning it won't compile if all cases aren't handled

    // use of the _ keyword to return any other values
    // () is just the unit value
    match some_i8_value {
        5 => println!("one"),
        _ => ()
    }

    // using match can be a bit wordy sometimes, so we can use
    // if let instead to shorten things up
    // if the logic is to verbose for match, use if let

    if let Some(12) = some_u8_value {
        println!("three");
    }
}
