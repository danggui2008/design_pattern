//AbstractClass【抽象类（Abstract Class）】
trait AbstractClass {
    //模板方法：定义骨架
    fn template_method(&self) {
        self.step1();
        self.step2();
        self.step3();
        self.step4();
    }

    fn step1(&self);
    fn step2(&self);
    fn step3(&self);
    fn step4(&self);
}

// ConcreteClass【具体类（Concrete Class）】
struct ConcreteClass;

impl AbstractClass for ConcreteClass {
    fn step1(&self) {
        println!("ConcreteClass step1");
    }
    fn step2(&self) {
        println!("ConcreteClass step2");
    }
    fn step3(&self) {
        println!("ConcreteClass step3");
    }
    fn step4(&self) {
        println!("ConcreteClass step4");
    }
}

#[test]
fn main() {
    let concrete_class = ConcreteClass;
    concrete_class.template_method();
}
