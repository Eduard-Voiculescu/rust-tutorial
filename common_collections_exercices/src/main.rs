use std::collections::HashMap;
use std::io;
use std::process::exit;

fn main() {
    match mean(&vec![1, 4, 74, 435, 99, 12, 66, 120]) {
        Some(mean) =>  {
            assert_eq!(mean, 101.375);
            println!("The mean of the list of integers is {}", mean);
        },
        None => println!("The mean of the list of integers doesn't exist")
    }

    match median(&vec![1, 4, 74, 435, 99, 12, 66, 120, 11]) {
        Some(median) => {
            assert_eq!(median, 74.0);
            println!("The median of the list of integers is {}", median);
        },
        None => println!("The median of the list of integers doesn't exist")
    }

    match mode (&vec![1, 5, 3, 7, 3, 5, 1, 3]) {
        Some(mode) => {
            assert_eq!(mode, 3.0);
            println!("The mode of the list of integers is {}", mode);
        },
        None => println!("The mode of the list of integers doesn't exit")
    }

    // small unit tests
    assert_eq!(to_pig_latin(&String::from("first")), "irst-fay ");
    assert_eq!(to_pig_latin(&String::from("apple")), "apple-hay ");
    assert_eq!(to_pig_latin(&String::from("My name is John")), "y-May ame-nay is-hay ohn-Jay ");

    let mut input = String::new();

    println!("Enter a text to convert to pig latin");

    io::stdin().read_line(&mut input)
        .expect("Failed to read from stdin");

    let pig_latin_text = to_pig_latin(&input);

    println!("The text is [{}] and in pig latin [{}]", input, pig_latin_text);

    start_company_software();
}

fn mean(list_of_integers: &Vec<i32>) -> Option<f64> {
    let sum: i32 = list_of_integers.iter().sum();
    let mean = sum as f64 / list_of_integers.len() as f64;

    return if mean.is_nan() {
        None
    } else {
        Some(mean)
    }
}

fn median(list_of_integers: &Vec<i32>) -> Option<f64> {
    let mut list_of_integers_copy = list_of_integers.clone();
    list_of_integers_copy.sort();

    let number_of_indexes = list_of_integers_copy.len();

    return if number_of_indexes % 2 == 0 { // number is even, take the mean of the 2 in the middle
        let lower_middle_index = number_of_indexes / 2 - 1;
        let higher_middle_index = number_of_indexes / 2 + 1;

        let new_list_of_integers: &Vec<i32> = &vec![list_of_integers_copy[lower_middle_index], list_of_integers_copy[higher_middle_index]];

        mean(new_list_of_integers)


    } else { // number is odd, return the value of the list at that position
        let middle_index = number_of_indexes / 2 + 1;

        Some(list_of_integers_copy[middle_index] as f64)
    }
}

fn mode(list_of_integers: &Vec<i32>) -> Option<f64> {
    let mut map = HashMap::new();

    let mut highest_occurrence: i32 = 0;
    let mut highest_occurrence_value = 0;

    for integer in list_of_integers {
        let count = map.entry(integer).or_insert(0);

        if count > &mut highest_occurrence {
            highest_occurrence = *count;
            highest_occurrence_value = *integer;
        }

        *count += 1;
    }

    Some(highest_occurrence_value as f64)
}

fn to_pig_latin(text: &str) -> String {
    let mut pig_latin_text = String::new();
    let mut chars: Vec<char>;

    for word in text.split_whitespace() {
        chars = word.chars().collect();

        let mut is_vowel = false;

        "aeiouyAEIOUY".chars().for_each(|vowel| {
            if &chars[0] == &vowel {
                is_vowel = true;
            }
        });

        chars.push('-');

        match is_vowel {
            true => {
                chars.push('h');
            }
            false => {
                let first_character = chars.remove(0);
                chars.push(first_character);
            }
        }

        chars.push('a');
        chars.push('y');

        let pig_latin_word: String = chars.iter().collect();
        pig_latin_text.push_str(&pig_latin_word);
        pig_latin_text.push_str(" ");

        chars.clear(); // clear the vector
    }

    pig_latin_text
}

fn start_company_software() {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    println!("Add departments, employees or fetch data by department or company.");

    loop {
        let mut input = String::new();

        println!();
        println!("Please choose any of the following options:");
        println!("1) Add a department.");
        println!("2) Add an employee.");
        println!("3) View all employees by department.");
        println!("4) View all employees in the company by department.");
        println!("q) Exit application.");

        io::stdin().read_line(&mut input)
            .expect("Failed to read from stdin");

        let input: char = match input.trim().parse() {
            Ok(inp) => inp, // all good, do nothing
            Err(_) => continue // as the input wasn't proper to what was asked
        };

        match input {
            '1' => add_department(&mut map),
            '2' => add_employee(&mut map),
            '3' => view_all_employees_by_department(&mut map),
            '4' => println!("Company directory: {:?}", map),
            'q' => exit(0),
            _ => panic!("Invalid input.")
        }
    }

}

fn add_department(map: &mut HashMap<String, Vec<String>>) {
    println!("Enter the name of the department.");

    let mut department = String::new();

    io::stdin().read_line( &mut department)
        .expect("Failed to read from stdin");

    let dept: String = match department.trim().parse() {
        Ok(dept) => dept,
        Err(_) => panic!("Invalid department input.")
    };

    map.entry(dept).or_insert(Vec::new());

    println!("Company map: {:?}", map);
    println!();
}

fn add_employee(map: &mut HashMap<String, Vec<String>>) {
    println!("Enter the name of the department.");

    let mut department = String::new();

    io::stdin().read_line( &mut department)
        .expect("Failed to read from stdin");

    let dept: String = match department.trim().parse() {
        Ok(dept) => dept,
        Err(_) => panic!("Invalid department input.")
    };

    let employees: Option<&mut Vec<String>> = map.get_mut(&dept);

    println!("Enter the name of the employee.");

    let mut employee_name= String::new();

    io::stdin().read_line(&mut employee_name)
        .expect("Failed to read from stdin");

    let empl_name: String = match employee_name.trim().parse() {
        Ok(empl) => empl,
        Err(_) => panic!("Invalid employee name input.")
    };

    if let Some(employees_vector) = employees {
        employees_vector.push(empl_name);
    } else {
        panic!("Invalid input.");
    }

    println!("Company map: {:?}", map);
    println!();
}

fn view_all_employees_by_department(map: &mut HashMap<String, Vec<String>>) {
    println!("Enter the name of the dept");

    let mut dept = String::new();

    io::stdin()
        .read_line(&mut dept)
        .expect("Failed to read line");

    let dept: String = match dept.trim().parse() {
        Ok(s) => s,
        Err(_) => panic!("invalid dept input"),
    };

    let employees = map.get(&dept);

    println!("Employees for dept: {}, are: {:?}", dept, employees);
}