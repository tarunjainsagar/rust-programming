fn main() {
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

    // Program 5
    define_var();

    // todo: Function, Shadowing
}

fn define_var() {
    let b: &str = "Hello Function!";
    println!("Program 5 Function - {}", b);
}
