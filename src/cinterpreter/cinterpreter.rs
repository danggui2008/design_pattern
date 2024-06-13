//Expression【抽象表达式（Abstract Expression）】
trait Expression {
    fn interpret(&self) -> i32;
}

//NumberExpression【终结符表达式（Terminal Expression）】
struct NumberExpression {
    value: i32,
}
impl NumberExpression {
    fn new(value: i32) -> Self {
        Self { value }
    }
}

//实现表达式接口：NumberExpression
impl Expression for NumberExpression {
    fn interpret(&self) -> i32 {
        self.value
    }
}

//加法表达式AddExpression【非终结符表达式（Non-terminal Expression）】
struct AddExpression {
    left_operand: Box<dyn Expression>,
    right_operand: Box<dyn Expression>,
}

impl AddExpression {
    fn new<T1, T2>(left_operand: T1, right_operand: T2) -> Self
    where
        T1: Expression + 'static,
        T2: Expression + 'static,
    {
        Self {
            left_operand: Box::new(left_operand),
            right_operand: Box::new(right_operand),
        }
    }
}
//实现表达式接口：AddExpression
impl Expression for AddExpression {
    fn interpret(&self) -> i32 {
        self.left_operand.interpret() + self.right_operand.interpret()
    }
}

//SubExpression减法表达式【非终结符表达式（Non-terminal Expression）】
struct SubExpression {
    left_operand: Box<dyn Expression>,
    right_operand: Box<dyn Expression>,
}

impl SubExpression {
    fn new<T1, T2>(left_operand: T1, right_operand: T2) -> Self
    where
        T1: Expression + 'static,
        T2: Expression + 'static,
    {
        Self {
            left_operand: Box::new(left_operand),
            right_operand: Box::new(right_operand),
        }
    }
}
//实现表达式接口：SubExpression
impl Expression for SubExpression {
    fn interpret(&self) -> i32 {
        self.left_operand.interpret() - self.right_operand.interpret()
    }
}

#[test]
fn main() {
    //3
    let number_expression = NumberExpression::new(3);
    //(10-3)
    let sub_expression = SubExpression::new(NumberExpression::new(10), NumberExpression::new(3));
    //5+(10-3)
    let add_expression = AddExpression::new(number_expression, sub_expression);
    //10
    let value = add_expression.interpret();
    println!("value:{}", value);
}
