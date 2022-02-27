pub fn run() {
    let age: u8 = 18;
    let check_id: bool = false;
    let knows_age: bool = true;
    
    if age >= 21 && check_id || knows_age {
        println!("Bartender: What would you like to drink?");
    } else  if age <21 && check_id {
        println!("Bartender: Too young - you gotta go!");
    } else {
        println!("Bartender: Show the ID!");
    }

    let is_of_age = if age >= 21 {true} else {false};
    println!{"Is of age? {}", is_of_age}
}