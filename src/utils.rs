use std::io;

// Collection of utillity functions used in main!

pub fn read_int() -> i32{
    let mut num = String::new(); 
    io::stdin()
        .read_line(&mut num)
        .expect("Could not read line...");
    let num: i32 = num.trim().parse().expect("Could not parse int!");
    num
    
}