pub fn run() {
    println!("Hello, from print.rs");
    println!("Casting numbers to string {}", 1);
    println!("{} is from {}", "Ryan", "WA");
    println!("{0} is from {1} and {0} likes to {2}", "Ryan", "WA", "code in Rust");
    println!("{name} likes to {activity}", name = "Ryan", activity = "draw buildings");
    println!("{:?}", (12, true, "hello"));
    println!("17 + 193 = {}", 17 + 193);
}