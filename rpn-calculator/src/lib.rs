#[derive(Debug, Clone)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

impl CalculatorInput {
    fn extract_value(&self) -> i32 {
        let mut value = 0;

        if let CalculatorInput::Value(x) = self {
            value = *x;
        };

        value
    }

    fn is_value(&self) -> bool {
        match self {
            CalculatorInput::Value(_) => true,
            _ => false
        }
    }
}

pub struct InputSelector {
    inputs: Vec<CalculatorInput>,
    calc: Vec<CalculatorInput>,
    index: usize
}

impl InputSelector {
    fn load_calc(&mut self) {
        let mut i = 0;

        while i < 3 && self.inputs.len() > 0 {
            self.calc.push(self.inputs.remove(self.index));
            i += 1
        }
    }

    fn eval(&mut self) {
        let a = self.calc[0].extract_value();
        let b = self.calc[1].extract_value();
        let operator = &self.calc[2];

        let result = match operator {
            CalculatorInput::Add => Some(a + b),
            CalculatorInput::Subtract => Some(a - b),
            CalculatorInput::Multiply => Some(a * b),
            CalculatorInput::Divide => Some(a / b),
            CalculatorInput::Value(x) => Some(*x)
        };

        self.inputs.insert(self.index, CalculatorInput::Value(result.unwrap()));

        if self.inputs.len() == 3 {
            self.index = 0;
        } else if self.calc.len() == 3 && self.index + 1 < self.inputs.len() {
            self.index += 1;
        } else {
            self.index = 0;
        }
    }

    fn is_done(&self) -> bool {
        if self.inputs.len() >= 3 {
            return false;
        }

        true
    }

    fn clear_calc(&mut self) {
        self.calc.clear();
    }

    fn get_result(&self) -> Option<i32> {
        if self.inputs.len() == 1 {
            let result = &self.inputs[0];

            if result.is_value() {
                return Some(result.extract_value());
            }
        }

        None
    }
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut selector = InputSelector {
        inputs: inputs.to_vec(),
        calc: Vec::new(),
        index: 0
    };

    while !selector.is_done() {
        selector.load_calc();
        selector.eval();
        selector.clear_calc();
    }

    selector.get_result()
}
