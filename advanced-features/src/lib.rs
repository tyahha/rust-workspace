use std::slice;

static HELLO_WORLD: &str = "Hello, world";
static mut COUNTER: u32 = 0;

unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}

pub fn advanced_features_main() {
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    unsafe {
        dangerous();
    }

    println!("name is: {}", HELLO_WORLD);

    unsafe {
        COUNTER += 1;
        println!("COUNTER: {}", COUNTER);
    }
}

extern "C" {
    fn abs(input: i32) -> i32;
}

unsafe fn dangerous() {
    println!("Absolute value of -3 according to C: {}", abs(-3));
}

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (slice::from_raw_parts_mut(ptr, mid),
         slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid))
    }
}