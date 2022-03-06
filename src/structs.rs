//create custom data types

//Traditional struct
// struct Color {
//     red: u8,
//     green: u8,
//     blue: u8
// }

//Tuple struct
// struct TupleColor (u8,u8,u8);

struct Person {
    first_name: String,
    last_name: String
}

//implement
impl Person {
    //construct the person
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }

    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    //Set last name
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    //name to tuple
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run() {
    // let mut color = Color {
    //     red: 255,
    //     green: 0,
    //     blue: 0
    // };
    // println!("Color: {} {} {}", color.red, color.green, color.blue);
    // color.blue = 100; 
    // println!("Color: {} {} {}", color.red, color.green, color.blue);

    // let mut tuple_color = TupleColor(255,100,0);
    // println!("Color: {} {} {}", tuple_color.0, tuple_color.1, tuple_color.2);
    // tuple_color.2 = 75;
    // println!("Color: {} {} {}", tuple_color.0, tuple_color.1, tuple_color.2);

        let mut person = Person::new("Ryan", "Gauthier");
        println!("Student: {} {}", person.first_name, person.last_name);
        println!("Student: {}", person.full_name());

        person.set_last_name("O'Malley");
        println!("Student: {}", person.full_name());

        // println!("Student: {:?}", person.to_tuple());
        let tuple_name = person.to_tuple();
        let tuple_last = tuple_name.1;

        println!("Student: {}", tuple_last);

}

