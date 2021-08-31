use std::io;

// Collection of utility functions used in main.rs!

pub fn read_float() -> f64 {
    let mut num = String::new();
    io::stdin()
        .read_line(&mut num)
        .expect("Could not read line...");
    let num: f64 = num.trim().parse().expect("Could not parse int!");
    num
}

pub fn read_int() -> i32 {
    let mut num = String::new();
    io::stdin()
        .read_line(&mut num)
        .expect("Could not read line...");
    let num: i32 = num.trim().parse().expect("Could not parse int!");
    num
}
