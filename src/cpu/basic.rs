#[no_mangle]
pub fn add_two(a: i32, b: i32) -> i32 {
    a + b
}

#[no_mangle]
pub fn add_ten(num: &mut i32) {
    *num += 10;
}
