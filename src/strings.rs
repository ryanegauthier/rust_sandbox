// Primitive str = Immutable fixed length string somewhere in memory
// String = Dynamic, heap-allocated data structure - used when you need to modify or own string data

pub fn run() {
    let hello = "Hello";
    println!("{}", hello);
    let mut hello2 = String::from("Hello");
    println!("Before push: {}", hello2);
    hello2.push_str(" World!");
    println!("After push: {}", hello2);
    println!("Capacity: {}", hello2.capacity());
    println!("Contains: {}", hello2.contains("World"));
    println!("Replace: {}", hello2.replace("World", "Everyone"));

    println!("Length: {}", hello2.len());
    for word in hello2.split_whitespace() {
        println!("{}", word);
    }

    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    println!("{}", s);
    
}