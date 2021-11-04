mod calculator;
mod utils;

use calculator::Calculator;

fn main() {
    println!("What would you like to do? \n");
    println!("1. add\n2. sub\n3. mult\n4. div\n");

    let choice = utils::read_int();
    match choice {
        1 => {
            let mut cal = Calculator::new();
            println!("Enter a number: ");
            cal.num1 = utils::read_float();
            println!("Enter a number: ");
            cal.num2 = utils::read_float();
            let res = cal.add();
            println!("Result: {}", res);
        }

        2 => {
            let mut cal = Calculator::new();
            println!("Enter a number: ");
            cal.num1 = utils::read_float();
            println!("Enter a number: ");
            cal.num2 = utils::read_float();
            let res = cal.sub();
            println!("Result: {}", res);
        }

        3 => {
            let mut cal = Calculator::new();
            println!("Enter a number: ");
            cal.num1 = utils::read_float();
            println!("Enter a number: ");
            cal.num2 = utils::read_float();
            let res = cal.mult();
            println!("Result: {}", res);
        }

        4 => {
            let mut cal = Calculator::new();
            println!("Enter a number: ");
            cal.num1 = utils::read_float();
            println!("Enter a number: ");
            cal.num2 = utils::read_float();
            let res = cal.div();
            println!("Result: {}", res);
        }

        _ => {
            println!("Not a valid operation");
        }
    }
}
