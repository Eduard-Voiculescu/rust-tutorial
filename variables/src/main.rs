use std::collections::LinkedList;

fn main() {
    let y: i32 = {
        let x: i32 = 3;
        x + 1
    };

    println!("The value of y is {}", y);

    let z = five();
    println!("The value of z is {}", z);

    let mut employee_list: LinkedList<&str> = LinkedList::new();

    let _ = &employee_list.push_back("element1");
    let _ = &employee_list.push_back("element2");
    let _ = &employee_list.push_back("element3");

    for element in employee_list {
        println!("{}", element);
    }
}

fn five () -> i32 {
    return 5;
}