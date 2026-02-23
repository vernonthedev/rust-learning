/*
* Primitive Types--
* Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (number of bits they take in memory)
* Floats: f32, f64
* Boolean (bool)
* Characters (char)
* Tuples
* Arrays
*/

pub fn run_types() {
    // Default "i32".
    let _x = 1;
    // Default "f64".
    let _d = 2.5;
    // Explicit type.
    let _y: i64 = 34343434343;
    // Getting bool from expression.
    let is_greater: bool = 10 > 7;

    // Finding the max sizes of the data types.
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    println!("{:?}", is_greater);
}
