pub fn runall() {
    default_types();
    type_conversions();
}

fn type_conversions() {
    let x: u16 = 32_u8 as u16;

    println!("Program 2 - Type Conversion x is {}", x);
}

fn default_types() {
    let x: i32 = 10;
    let mut y = 20; // Default Type i32

    y = x;

    println!("Program 1 - Default integer type i32")
}
