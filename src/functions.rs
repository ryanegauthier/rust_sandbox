pub fn run() {
    greeting("Hola", "Sara");
    let get_sum = add(7, 21);
    println!("7 + 21 = {}", get_sum);

    //Closure
    let n3: i32 = 4;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("C nums sum: {}", add_nums(12, 5));


}

fn greeting(greet: &str, name: &str) {
    println!("{} {}, nice to meet you!", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}

