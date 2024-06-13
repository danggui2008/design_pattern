//咖啡接口【组件（Component）】
trait Coffee {
    fn cost(&self) -> f64;
    fn desc(&self) -> String;
}

//普通咖啡【具体组件（Concrete Component）】
struct SimpleCoffee {}

//实现咖啡接口
impl Coffee for SimpleCoffee {
    fn cost(&self) -> f64 {
        10_f64
    }
    fn desc(&self) -> String {
        "Simple Coffee".to_string()
    }
}

//CoffeeDecorator接口【装饰器（Decorator)】
trait CoffeeDecorator: Coffee {
}
//MilkDecorator【具体装饰器（Concrete Decorator）】
struct MilkDecorator {
    coffee: Box<dyn Coffee>,
}

impl MilkDecorator {
    fn new<T>(coffee: T) -> Self
    where
        T: Coffee + 'static,
    {
        Self {
            coffee: Box::new(coffee),
        }
    }
}

impl Coffee for MilkDecorator {
    fn cost(&self) -> f64 {
        self.coffee.cost() + 2_f64
    }
    fn desc(&self) -> String {
        format!("{} ,with  Milk", self.coffee.desc())
    }
}
impl CoffeeDecorator for MilkDecorator{}

//SugarDecorator【具体装饰器（Concrete Decorator）】
struct SugarDecorator {
    coffee: Box<dyn Coffee>,
}

impl SugarDecorator {
    fn new<T>(coffee: T) -> Self
    where
        T: Coffee + 'static,
    {
        Self {
            coffee: Box::new(coffee),
        }
    }
}

impl Coffee for SugarDecorator {
    fn cost(&self) -> f64 {
        self.coffee.cost() + 2_f64
    }
    fn desc(&self) -> String {
        format!("{} ,with  Sugar", self.coffee.desc())
    }
}
impl CoffeeDecorator for SugarDecorator {
}
#[test]
fn main() {
    //普通咖啡
    let simple_coffee = SimpleCoffee {};
    println!(
        "花了{}(元)，买了一杯{}",
        simple_coffee.cost(),
        simple_coffee.desc()
    );
    //加牛奶的咖啡
    let milk_decorator = MilkDecorator::new(simple_coffee);
    println!(
        "花了{}(元)，买了一杯{}",
        milk_decorator.cost(),
        milk_decorator.desc()
    );
    //给牛奶的咖啡加点糖
    let sugar_decorator = SugarDecorator::new(milk_decorator);
    println!(
        "花了{}(元)，买了一杯{}",
        sugar_decorator.cost(),
        sugar_decorator.desc()
    );
}
