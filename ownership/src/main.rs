fn main() {
    let mut s = String::from("hello");

    println!("{}", s);
    println!("Let's append data to our String.");

    s.push_str(", world!");

    println!("{}", s);

    let str1: &str = "str1";
    let str2 = str1;

    // this will work because we use string literals
    println!("str1: {}, str2: {}", str1, str2);

    let string1: String = String::from("String");
    let string2: String = string1;

    // this won't work because we use the data type String
    // and string1 will be unavailable the moment we assign
    // string 2 = string 1
    //println!("string1: {}, string2: {}", string1, string2)

    let s1 = String::from("Tata");
    let (s2, len) = calculate_length_tuple(s1);

    // this print statement will fail because the variable s1 has been moved
    //println!("{}", s1);
    println!("The length of '{}' is {}.", s2, len);

    let s1 = String::from("Hello");
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    // mutable references
    let mut s: String = String::from("hello");
    println!("String s before change: {}", s);

    change(&mut s);
    println!("String s after  change: {}", s);

    // important note on mutability of references
    // -> we cannot reference twice the same instance
    // except if we pass in a macro or method between references
    let mut s = String::from("hello");

    let r1 = &mut s;
    // println!("{}", r1); // this or
    do_nothing(&mut s); // this will work
    let r2 = &mut s; // this will fail, because we can't reference s a second time
    println!("{}", r2);

    // multiple immutable references is ok, because we are only reading the date
    // and no one is write data to it
    let mut mutable1 = String::from("mutable1");

    let reference1 = &mutable1;
    let reference2 = &mutable1;

    // no good, will cause big problem, we can call a macro or method to have the scope be affected
    // and the reference1 and reference2 will be dropped and then we can use the below statement
    // let reference3 = &mut mutable1;

    // Rules of references
    // * At any given time, you can have either one mutable reference or any number of immutable references.
    // * References must always be valid.
}

fn calculate_length_tuple(s: String) -> (String, usize) {
    let length = s.len();

    return (s, length);
}

fn calculate_length(s: &String) -> usize {
    return s.len();
}

fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}

fn do_nothing(some_string: &mut String) {
    // do nothing
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    return &s[..]
}
