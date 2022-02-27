pub fn run() {
    // let mut count = 0;

    // loop {
    //     count += 1;

    //     if count < 20{ 
    //         println!("{}, Mississippi", count);
    //     } else {
    //         println!("{}! Ready or not, here I come!", count);
    //         break;
    //     }
    // }

    // while count <= 100 {
    //     count += 1;
    //     if count %3 == 0 && count %5 == 0 {
    //         println!("fizzbuzz");
    //     } else if count %3 == 0 {
    //         println!("fizz");
    //     } else if count %5 == 0 {
    //         println!("buzz");
    //     } else {
    //         println!("{}", count);
    //     }
    // }
    for count in 0..100 {
        if count %3 == 0 && count %5 == 0 {
            println!("fizzbuzz");
        } else if count %3 == 0 {
            println!("fizz");
        } else if count %5 == 0 {
            println!("buzz");
        } else {
            println!("{}", count);
        }
    }
}