// 4_structs.rs

// Structs are custom data types that let you name and package together multiple related values.

// 1. Define a simple struct
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    // 2. Create an instance of the struct
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    println!("User 1 email: {}", user1.email);
    println!("User 1 username: {}", user1.username);

    // 3. Mutability: To change a value in a struct instance, the entire instance must be mutable.
    let mut user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername"),
        active: true,
        sign_in_count: 1,
    };

    user2.email = String::from("newemail@example.com");
    println!("User 2 new email: {}", user2.email);

    // 4. Struct Update Syntax: Create new instances from old ones with some values changed.
    let user3 = User {
        email: String::from("third@example.com"),
        username: String::from("thirdusername"),
        ..user1 // This means the remaining fields (active, sign_in_count) get their values from user1
    };
    println!("User 3 sign in count: {}", user3.sign_in_count);

    // 5. Tuple Structs: Structs without named fields. Useful when you want to give the whole tuple a name.
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("Black color R: {}", black.0);
    println!("Origin point X: {}", origin.0);

    // 6. Unit-Like Structs: Structs without any fields. Useful for implementing traits.
    struct AlwaysEqual;
    let subject = AlwaysEqual;

    // Example function using a struct
    fn build_user(email: String, username: String) -> User {
        User {
            email, // Field Init Shorthand: If parameter name and field name are the same.
            username,
            active: true,
            sign_in_count: 1,
        }
    }

    let user4 = build_user(
        String::from("fourth@example.com"),
        String::from("fourthusername"),
    );
    println!("User 4 email: {}", user4.email);
}
