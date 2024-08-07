#![allow(dead_code)]
#![allow(unused_variables)]

struct Book {
    title: String,
    author: String,
    pages: u32,
    isbn: String,
    available: bool
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

fn main() {
    // Create a new book
    let book: Book = Book {
        title: String::from("The Catcher in the Rye"),
        author: String::from("J.D. Salinger"),
        pages: 234,
        isbn: String::from("978-0-316-76948-0"),
        available: true
    };

    // Create a new user
    let mut user: User = User {
        username: String::from("johndoe"),
        email: String::from("john@doe.com"),
        sign_in_count: 1,
        active: true
    };

    // Update the user's email
    user.email = String::from("johndoe@mail.com");

    println!("User.Email: {}", user.email);

    // Create a new user using a function
    let user2: User = build_user(String::from("janedoe"), String::from("jane@doe.com"));

    println!("User2.Username: {}", user2.username);

    // Create a new user using a function with field init shorthand
    let user3 = User {
        email: String::from("janedoe@mail.com"),
        ..user2
    };

    println!("User3.Username: {}", user3.username);
    println!("User3.Email: {}", user3.email);

    // Tuple Structs
    struct Color(u8, u8, u8);

    let black: Color = Color(0, 0, 0);
    let white: Color = Color(255, 255, 255);

    println!("Black: ({}, {}, {})", black.0, black.1, black.2);
    println!("White: ({}, {}, {})", white.0, white.1, white.2);

    // Unit-Like Structs
    struct AlwaysEqual;

    let unit_like: AlwaysEqual = AlwaysEqual;
}

fn build_user(username: String, email: String) -> User {
    User {
        username,
        email,
        sign_in_count: 1,
        active: true
    }
}
