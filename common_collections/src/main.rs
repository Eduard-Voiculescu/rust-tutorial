fn main() {
    let v: Vec<i32> = Vec::new();
    let v1 = vec![1, 2, 3];

    let mut v2 = Vec::new();
    v2.push(5);
    v2.push(6);
    v2.push(7);
    v2.push(8);

    let v3 = vec![1, 2, 3, 4 ,5];

    let third: &i32 = &v3[2];
    println!("The third element is {}", third);

    match v3.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element")
    }

    //let does_not_exist = &v[100]; // this will cause the program to panic
    let does_not_exist = v.get(100); // won't panic and will return a None

    let mut v5 = vec![1, 2, 3, 4, 5];

    let first = &v5[0];

    // This error is due to the way vectors work: adding a new element onto the end of the vector might require allocating new memory and copying the old elements to the new space, if there isn’t enough room to put all the elements next to each other where the vector currently is. In that case, the reference to the first element would be pointing to deallocated memory
    // v5.push(6);

    println!("The first element is: {}", first);

    let v6 = vec![100, 32, 57];
    for i in &v6 {
        println!("{}", i);
    }

    let mut v7 = vec![100, 32, 57];
    for i in &mut v7 {
        *i += 50;
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("blue"))
    ];
}
