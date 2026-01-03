//
const ONE_MINUTE: i32 = 60;
pub fn name() {
    let beverage = "Coffee";
    println!("I love {}", beverage);
    println!("How many is {}", ONE_MINUTE);
}

// int types in rust
pub fn int_types() {
    let n1: i8 = 10; // signed ones can be negative -128 to 127
    let n2: u32 = 200; // unsigned values are always positive,0,255
    println!("Both values are {},{}", n1, n2);
}

pub fn float_types() {
    let a: f64 = 0.5;
    let b: f64 = 0.5;
    let c = a + b;

    println!("{}", c)
}

// Bools and chars you know
