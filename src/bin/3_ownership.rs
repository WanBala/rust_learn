fn main() {
    // 1. Ownership
    ownership();

    // 2. Borrowing and References
    borrowing_and_references();

    // 3. Slices
    slices();
}

fn ownership() {
    // Ownership is Rust's most unique feature. It enables Rust to make memory safety
    // guarantees without needing a garbage collector.

    // 1. Each value in Rust has a variable that's called its owner.
    // 2. There can only be one owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped.

    // Example 1: Move
    let s1 = String::from("hello");
    let s2 = s1; // s1 is moved to s2. s1 is no longer valid.
                 // println!("{}, world!", s1); // This would cause a compile error.
    println!("{}, world!", s2);

    // Example 2: Clone (deep copy)
    let s3 = String::from("hello");
    let s4 = s3.clone(); // s3 is cloned to s4. Both are valid.
    println!("s3 = {}, s4 = {}", s3, s4);

    // Ownership with functions
    let s = String::from("hello");
    takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here.
                        // println!("{}", s); // This would cause a compile error.

    let x = 5;
    makes_copy(x); // x would move into the function,
                   // but i32 is Copy, so it's okay to still
                   // use x afterward.
    println!("x is still valid: {}", x);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing memory is freed.

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn borrowing_and_references() {
    // Instead of transferring ownership, you can let a function "borrow" a value.
    // This is done by passing a reference to the value.

    let s1 = String::from("hello");
    let len = calculate_length(&s1); // &s1 creates a reference to s1.
    println!("The length of '{}' is {}.", s1, len); // s1 is still valid here.

    // Mutable References
    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s);

    // Rules of References:
    // 1. At any given time, you can have either one mutable reference or any number of immutable references.
    // 2. References must always be valid.
}

fn calculate_length(s: &String) -> usize {
    // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what it refers to, nothing happens.

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn slices() {
    // A slice lets you reference a contiguous sequence of elements in a collection
    // rather than the whole collection.

    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{} {}", hello, world);

    let first = first_word(&s);
    println!("the first word is: {}", first);

    // Array slices
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
