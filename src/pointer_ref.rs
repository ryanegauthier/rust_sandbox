pub fn run() {
    // let arr1 = [1,2,3,4,5];
    // let arr2 = arr1;
    // println!("Values: {:?}", (arr1, arr2));

    //non primitive values need to be referenced to - by using the &
    let vec1 = vec![1,2,3,4,5];
    let vec2 = &vec1;
    println!("Values: {:?}", (&vec1, vec2));


}

