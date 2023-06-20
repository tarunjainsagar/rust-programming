pub fn runall() {
    default_types();
    type_conversions();
    infer_type();
    max_ranges();
}

fn max_ranges() {
    println!(
        "Program 4 - Max Ranges of i8 is {} and u8 is {}",
        i8::MAX,
        u8::MAX
    );
}

#[allow(unused_variables)]
fn infer_type() {
    let x: u32 = 5;
    let y = -10;

    println!(
        "Program 3 - Infer Type x is of type {}, y is of type {}",
        type_of(&x),
        type_of(&y)
    );
}

fn type_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}

fn type_conversions() {
    let x: u16 = 32_u8 as u16;

    println!("Program 2 - Type Conversion x is {}", x);
}

#[allow(unused_variables, unused_assignments)]
fn default_types() {
    let x: i32 = 10;
    let mut y = 20; // Default Type i32

    y = x;

    println!("Program 1 - Default integer type i32")
}
