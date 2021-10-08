mod utils;

struct Calculator {
    num1: f64,
    num2: f64,
}

impl Calculator {
    // sets the default value for num1 and num2 to 0
    fn new() -> Self {
        Calculator {
            num1: 0.0,
            num2: 0.0,
        }
    }

    fn add(&self) -> f64 {
        self.num1 + self.num2
    }

    fn sub(&self) -> f64 {
        self.num1 - self.num2
    }

    fn mult(&self) -> f64 {
        self.num1 * self.num2
    }

    fn div(&self) -> f64 {
        self.num1 / self.num2
    }
}

fn main() {
    println!("What would you like to do? \n");
    println!("1. add\n2. sub\n3. mult\n4. div\n");

    let choice = utils::read_int();
    match choice {
        1 => {
            let mut cal = Calculator::new();
            println!("Enter the first number: ");
            cal.num1 = utils::read_float();
            println!("Enter the second number: ");
            cal.num2 = utils::read_float();
            let res = cal.add();
            println!("Result: {}", res);
        }

        2 => {
            let mut cal = Calculator::new();
            println!("Enter the first number: ");
            cal.num1 = utils::read_float();
            println!("Enter the second number: ");
            cal.num2 = utils::read_float();
            let res = cal.sub();
            println!("Result: {}", res);
        }

        3 => {
            let mut cal = Calculator::new();
            println!("Enter the first number: ");
            cal.num1 = utils::read_float();
            println!("Enter the second number: ");
            cal.num2 = utils::read_float();
            let res = cal.mult();
            println!("Result: {}", res);
        }

        4 => {
            let mut cal = Calculator::new();
            println!("Enter the first number: ");
            cal.num1 = utils::read_float();
            println!("Enter the second number: ");
            cal.num2 = utils::read_float();
            let res = cal.div();
            println!("Result: {}", res);
        }
        _ => {
            println!("Not a valid operation");
        }
    }
}
