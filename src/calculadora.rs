use std::str::FromStr;

#[derive(Default)]
pub struct Calculator {
    value: u8,
}

#[derive(PartialEq, Eq, Debug)]
pub enum Operation {
    Add(u8),
    Sub(u8),
    Mul(u8),
    Div(u8),
}

impl FromStr for Operation {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Split the string into tokens separated by whitespace.
        let tokens: Vec<&str> = s.split_whitespace().collect();

        // Try to convert the vector into a statically-sized array of 2 elements, failing otherwise.
        let [_codigo, operation, operand] = tokens.try_into().map_err(|_| "Operación invalida")?;

        // Parse the operand into an u8.
        let operand: u8 = operand.parse().map_err(|_| "Operación invalida")?;

        match operation {
            "+" => Ok(Operation::Add(operand)),
            "-" => Ok(Operation::Sub(operand)),
            "*" => Ok(Operation::Mul(operand)),
            "/" => Ok(Operation::Div(operand)),
            _ => Err("Operación invalida"),
        }
    }
}

impl Calculator {
    pub fn value(&self) -> u8 {
        self.value
    }

    pub fn apply(&mut self, op: Operation) {
        match op {
            Operation::Add(operand) => self.value = self.value.wrapping_add(operand),
            Operation::Sub(operand) => self.value = self.value.wrapping_sub(operand),
            Operation::Mul(operand) => self.value = self.value.wrapping_mul(operand),
            Operation::Div(operand) => self.value = self.value.wrapping_div(operand),
        }
    }
}
