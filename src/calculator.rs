pub struct Calculator {
    pub num1: f64,
    pub num2: f64
}

impl Calculator {
    // sets the default value for num1 and num2 to 0
    pub fn new() -> Self {
        Calculator {
            num1: 0.0,
            num2: 0.0,
        }
    }

    pub fn add(&self) -> f64 {
        self.num1 + self.num2
    }

    pub fn sub(&self) -> f64 {
        self.num1 - self.num2
    }

    pub fn mult(&self) -> f64 {
        self.num1 * self.num2
    }

    pub fn div(&self) -> f64 {
        self.num1 / self.num2
    }
}
