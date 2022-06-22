/*
Primitive types:
Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (number of bits they take in memory)
Floats: f32, f64
Boolean (bool)
Characters (char)
Tuples
Arrays
*/

pub fn run() {
    // integer default is i32
    let x = 1;

    // float default is f64
    let y = 2.5;

    // explicit type
    let z: i64 = 45454545454545;

    // find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    // boolean
    let is_active: bool = true;

    // get boolean from expression
    let is_greater = 10 < 5;

    // Character (use single quotes)
    let a1 = 'a';

    // unicode
    let face = '\u{1F920}';
    let warn = '\u{26A0}';

    println!("{:?}", (x, y, z, is_active, is_greater, a1, face, warn));
}
