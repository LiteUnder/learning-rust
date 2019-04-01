// so structs are a thing
// a struct stores related data
// it's kinda like a mini class but also not cause it only stores data
// a structs name is camel cased, and order doesn't matter
// uses key -> value pairs

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// create instance of a struct
let user1 = User {
    email: String::from("bigfatgay@stupid.co.uk"),
    username: String::from("bigfatgayman"),
    active: false,
    sign_in_count: 7721,
};

// Structs may not be classes, but you can make constructors for them
// behaves exactly the same but removes repetition of email and username
fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

// struct update syntax
// ..user1 grabs same valeus from user1 instance for active and sign_in_count
let user2 = User {
    email: String::from("user2@user.com"),
    username: String::from("yetAnotherUserBoi"),
    ..user1
};
