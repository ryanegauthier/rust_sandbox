//Fixed length list where elements are the same data types
use std::mem;
pub fn run() { 
    let mut numbers: Vec<i32> = vec![1,2,3,4,5];
    println!("{:?}", numbers);
    println!("First value: {}", numbers[0]);
    
    numbers[2] = 22;
    println!("{:?}", numbers);
    println!("New value: {}", numbers[2]);

    numbers.push(6);
    numbers.push(7);
    
    println!("{:?}", numbers);
    println!("New value: {}", numbers[2]);
    numbers.pop();
    println!("{:?}", numbers);

    println!("Length: {}", numbers.len());

    println!("Numbers takes up {} bytes", mem::size_of_val(&numbers));

    //a slice contains [first, end) elements of an array 
    let slice: &[i32] = &numbers[1..3];
    println!("{:?}", slice);

    for number in numbers.iter() {
        println!("Number: {}", number);
    }
    for number in numbers.iter_mut() {
        *number *= 2;
        println!("Number: {}", number);
    }
    println!("{:?}", numbers);
}