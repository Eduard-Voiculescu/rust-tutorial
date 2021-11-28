use std::collections::HashMap;

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

    // ----- Strings ----- \\
    let mut s = String::new();

    let data = "initial contents";

    let s = data.to_string();

    // the method also works on a literal directly:
    let s = "initial contents".to_string();

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);

    // ----- HashMap ----- \\
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    match score {
        Some(&i32) => println!("Value for [{}] is {:?}", team_name, score),
        None => println!("Value [{}] does not exist", team_name)
    }

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);

    scores.entry(String::from("Yellow")).or_insert(80);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut scores: HashMap<_, _> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map: HashMap<String, String> = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        // this will change the value of the k, v because or_insert returns
        // a mutable reference. So the first time a word is added, we put 0
        // but then we do count += 1 so 1 is placed in. Then if we see the
        // word again, we would get 1 as count and the count += 1 would actually
        // put 2 as the value for the given key
        *count += 1;
    }

    println!("{:?}", map);
}
