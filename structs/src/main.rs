struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

fn main() {
    let user1 = build_user(String::from("user1@gmail.com"), String::from("user1"));

    let user2 = User {
        email: String::from("user2@gmail.com"),
        ..user1
    }; // can't use user1 username after this line, because it was moved to user 2

    println!("user1 email: {}", user1.email); // ok, because the value wasn't moved to user 2
    // println!("user1 username: {}", user1.username); // won't work
    println!("user1 active: {}", user1.active); // ok, implements the Copy trait
    println!("user1 sign_in_count: {}", user1.sign_in_count); // ok, implements the Copy trait
    println!("user2 username: {}", user2.username); // ok

    // Tuple Structs
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // Unit-Like Structs
    struct AlwaysEqual;

    let subject = AlwaysEqual;
}

fn defining_and_instantiating_structs() {

}

fn build_user(email: String, username: String) -> User {
    return User {
        email,
        username,
        active: true,
        sign_in_count: 1
    };
}