// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap allocated data structure - Use when you need to modify or own string data

pub fn run() {
    // A UTF-8–encoded, String slice, Immutable fixed-length string.
    let hello_1: &str = "Hello 1";
    println!("{}", hello_1);

    // Get length
    println!("Length: {}", hello_1.len());

    // A UTF-8–encoded, growable string.
    let mut hello_2: String = String::from("Hello 2");
    println!("{}", hello_2);
    // Push char
    hello_2.push('a');
    println!("{}", hello_2);
    // Push
    hello_2.push_str(" world!");
    println!("{}", hello_2);

    println!("Length: {}", hello_2.len());

    // Get capacity in bytes
    println!("Capacity: {}", hello_2.capacity());

    // Check if empty string
    println!("Is Empty: {}", hello_2.is_empty());

    // Contains
    println!("Contains 'world' {}", hello_2.contains("world"));

    // Replace
    println!("Replace: {}", hello_2.replace("world", "there"));
    println!("{}", hello_2);

    hello_2 = hello_2.replace("world", "there");
    println!("{}", hello_2);

    // Loop trough string by whitespace
    for word in hello_2.split_whitespace() {
        println!("{}", word);
    }

    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    println!("{}", s);

    // Assertion testing
    assert_eq!(2, s.len());
    assert_eq!(11, s.capacity());
}
