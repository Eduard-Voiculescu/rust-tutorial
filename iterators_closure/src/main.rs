use std::collections::HashMap;
use std::thread;
use std::time::Duration;

struct Cacher<T>
where T: Fn(u32) -> u32 {
    calculation: T,
    value: HashMap<u32, Option<u32>>
}

impl<T> Cacher<T>
where T: Fn(u32) -> u32 {
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: HashMap::new()
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        if self.value.contains_key(&arg) {
            self.value.get(&arg).unwrap().unwrap()
        } else {
            let v = (self.calculation)(arg);
            self.value.insert(arg, Some(v));
            v
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_closure = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} push ups!", expensive_closure.value(intensity));
        println!("Next, do {} sit ups!", expensive_closure.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_closure.value(intensity));
        }
    }
}

fn main() {
    // let simulated_user_specific_value = 10;
    // let simulated_random_number = 7;
    //
    // generate_workout(simulated_user_specific_value, simulated_random_number);
    //
    // let x = vec![1, 2, 3];
    //
    // let equal_to_x = move |z| z == x;
    //
    // // println!("Can't use x here: {:?}", x); // variable was moved, so if we uncomment this it will not compile
    //
    // let y = vec![1, 2, 3];
    //
    // assert!(equal_to_x(y));

    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<i32> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_with_different_values() {
        let mut c = Cacher::new(|a| a);

        let v1 = c.value(1);
        let v2 = c.value(2);

        assert_eq!(v2, 2);
    }
}
