pub fn runall() {
    // Program 1 - hello world
    println!("Program 1 - Hello, world!");

    // Program 2 - Variables
    let x: i32 = 5;
    let _y: i32; // uninitialized variable

    assert_eq!(x, 5);
    println!("Program 2 Variable - Success!");

    // Program 3 - Mutuable variable
    let mut a: i32 = 1;
    a += 2;

    assert_eq!(a, 3);
    println!("Program 3 Mutable Variable - Success!");

    // Program 4 - Scope
    {
        let y: i32 = 10;
        println!("Value of a is {} and y is {}", a, y);
    }
    println!("Program 4 Scope - Success!");

    // Program 5 - Functions
    define_var();

    // Program 6 - Shadowing
    shadowing();

    // Program 7 - Destructuring
    destructuring();

    // Program 8 - Tuple, Slice, Struct Patterns
    tuple_slice();
}

fn tuple_slice() {
    let (x, y); //tuple
    (x, ..) = (3, 4); // Slice
    [.., y] = [1, 2]; // Slice
    println!("Program 8 Tuple Slice - x is {} and y is {}", x, y);
}

fn destructuring() {
    let (mut x, y) = (10, 20);
    x += 3;

    assert_eq!(x, 13);
    assert_eq!(y, 20);
    println!("Program 7 Destructuring - x is {} and y is {}", x, y);
}

#[allow(unused_variables)]
fn shadowing() {
    let x: i32 = 5;

    {
        let x: i32 = 12;
        assert_eq!(x, 12);
    }

    assert_eq!(x, 5);

    // shadowing
    let x: i32 = 42;

    // another shadowing
    let y: i32 = 50;
    let y: &str = "I am shadowed";
    println!("Program 6 Shadowning - x is {} and y is {}", x, y);
}

fn define_var() {
    let b: &str = "Hello Function!";
    println!("Program 5 Function - {}", b);
}
