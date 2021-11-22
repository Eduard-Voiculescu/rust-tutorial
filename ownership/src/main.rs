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
}

fn calculate_length_tuple(s: String) -> (String, usize) {
    let length = s.len();

    return (s, length);
}

fn calculate_length(s: &String) -> usize {
    return s.len();
}
