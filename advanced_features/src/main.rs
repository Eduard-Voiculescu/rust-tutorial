use std::slice;

static HELLO_WORLD : &str = "Hello, world!";

static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    unsafe {
        dangerous();
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    println!("name is: {}", HELLO_WORLD);

    add_to_count(3);

    unsafe {
        // statics are considered unsafe because in a multi-threaded context
        // accessing the data will result in race conditions
        println!("COUNTER: {}", COUNTER);
    }
}

unsafe fn dangerous() {}

// check https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html#creating-a-safe-abstraction-over-unsafe-code
fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            // basically: split slice at ptr location and get mid number of elements
            // then for the second slide, we take ptr and offset to mid number of elements
            // and add the remaining elemnts (len - mid)
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid)
        )
    }
}

extern "C" {
    fn abs(input: i32) -> i32;
}