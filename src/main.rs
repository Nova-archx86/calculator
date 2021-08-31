mod operations;
mod utils;

fn main() {
    println!("What would you like to do? \n");
    println!("1. add\n2. sub\n3. mult\n4. div\n");

    let choice = utils::read_int();
    match choice {
        1 => {
            println!("Enter the first number: ");
            let num1: i32 = utils::read_int();
            println!("Enter the second number");
            let num2: i32 = utils::read_int();

            let res = operations::add(num1, num2);
            println!("Result: {}", res);
        }
        2 => {
            println!("Enter the first number: ");
            let num1: i32 = utils::read_int();
            println!("Enter the second number: ");
            let num2: i32 = utils::read_int();

            let res = operations::sub(num1, num2);
            println!("Result: {}", res);
        }

        3 => {
            println!("Enter the first number: ");
            let num1: i32 = utils::read_int();
            println!("Enter the second number: ");
            let num2: i32 = utils::read_int();

            let res = operations::mult(num1, num2);
            println!("Result: {}", res);
        }

        4 => {
            println!("Enter the first number: ");
            let num1: i32 = utils::read_int();
            println!("Enter the second number: ");
            let num2: i32 = utils::read_int();

            let res = operations::div(num1, num2);
            println!("Result: {}", res);
        }

        _ => {
            println!("Not a valid operation");
        }
    }
}
