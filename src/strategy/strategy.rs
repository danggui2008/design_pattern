//MathOperation【抽象策略类（Strategy）】
trait MathOperation {
    fn operate(&self, a: f32, b: f32) -> f32;
}
//Addition【具体策略类ConcreteStrategy】
struct Addition {}

//加法：实现策略接口
impl MathOperation for Addition {
    fn operate(&self, a: f32, b: f32) -> f32 {
        a + b
    }
}

//【具体策略类ConcreteStrategy】
struct Subtraction {}

//MathOperation减法：实现策略接口
impl MathOperation for Subtraction {
    fn operate(&self, a: f32, b: f32) -> f32 {
        a - b
    }
}

//Multiplication【具体策略类ConcreteStrategy】
struct Multiplication {}

//乘法：实现策略接口
impl MathOperation for Multiplication {
    fn operate(&self, a: f32, b: f32) -> f32 {
        a * b
    }
}

//Calculator【上下文Context】
struct Calculator {
    operation: Box<dyn MathOperation>,
}
//计算：可设置不同的策略接口
impl Calculator {
    fn new<T>(operation: T) -> Self
    where
        T: MathOperation + 'static,
    {
        Self {
            operation: Box::new(operation),
        }
    }

    fn set_operation<T>(&mut self, operation: T)
    where
        T: MathOperation + 'static,
    {
        self.operation = Box::new(operation);
    }

    fn perform_operation(&self, a: f32, b: f32) -> f32 {
        self.operation.operate(a, b)
    }
}

#[test]
fn main() {
    let mut calculator = Calculator::new(Addition {});
    let result = calculator.perform_operation(20.2, 550.20);
    println!("result:{}", result);
    calculator.set_operation(Subtraction {});
    let result = calculator.perform_operation(99.90, 88.52);
    println!("result:{}", result);

    calculator.set_operation(Multiplication {});
    let result = calculator.perform_operation(100.00, 88.52);
    println!("result:{}", result);
}
