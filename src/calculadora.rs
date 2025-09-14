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
        let tokens: Vec<&str> = s.split_whitespace().collect();
        let [_codigo, operation, operand] = tokens.try_into().map_err(|_| "parsing error")?;
        let operand: u8 = operand.parse().map_err(|_| "parsing error")?;

        if operation == "/" && operand == 0 {
            return Err("division by zero");
        }

        match operation {
            "+" => Ok(Operation::Add(operand)),
            "-" => Ok(Operation::Sub(operand)),
            "*" => Ok(Operation::Mul(operand)),
            "/" => Ok(Operation::Div(operand)),
            _ => Err("parsing error"),
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

#[cfg(test)]
mod tests {
    //use crate::calculadora::*;

    use super::*;
   

    #[test]
    fn operacion_suma_funciona() {
        let mut calculadora = Calculator::default();
        let operation = Operation::from_str(&"OP + 3".to_string()).unwrap();
        calculadora.apply(operation);
        assert_eq!(calculadora.value, 3);

    }

    #[test]
    fn operacion_resta_funciona() {
        let mut calculadora = Calculator::default();
        let suma = Operation::from_str(&"OP + 3".to_string()).unwrap();
        let resta = Operation::from_str(&"OP - 2".to_string()).unwrap();
        calculadora.apply(suma);
        calculadora.apply(resta);
        assert_eq!(calculadora.value, 1);


    }

    #[test]
    fn operacion_multiplicacion_funciona() {
        let mut calculadora = Calculator::default();
        let suma = Operation::from_str(&"OP + 3".to_string()).unwrap();
        let multiplicacion = Operation::from_str(&"OP * 2".to_string()).unwrap();
        calculadora.apply(suma);
        calculadora.apply(multiplicacion);
        assert_eq!(calculadora.value, 6);
    }

    #[test]
    fn operacion_division_funciona() {
        let mut calculadora = Calculator::default();
        let suma = Operation::from_str(&"OP + 10".to_string()).unwrap();
        let division = Operation::from_str(&"OP / 2".to_string()).unwrap();
        calculadora.apply(suma);
        calculadora.apply(division);
        assert_eq!(calculadora.value, 5);

    }

    #[test]
    fn dividir_por_cero_da_error() {
        assert!(Operation::from_str(&"OP / 0".to_string()).is_err());

    }
}