pub fn run() {
    let name = "Ryan";
    let mut age = 42;
    println!("My name is {} and I am {}", name, age);

    age = 43;

    println!("My name is {} and I am now {}", name, age);

    let (my_name, my_age) = ("Ryan", 42);
    println!("{} is {}", my_name, my_age);
}