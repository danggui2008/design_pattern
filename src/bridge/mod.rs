//颜色接口【实现（Implementor）】
trait Color {
    fn apply_color(&self);
}
//Red【具体实现（ConcreteImplementor）】
struct Red {}
impl Color for Red {
    fn apply_color(&self) {
        println!("applying red color");
    }
}

//实现部分Blue
struct Blue {}

impl Color for Blue {
    fn apply_color(&self) {
        println!("applying blue color")
    }
}

//shape接口【抽象（Abstraction）】
trait Shape {
    fn draw(&self);
}
//Circle【修正抽象（RefinedAbstraction）】
struct Circle {
    color: Box<dyn Color>,
}

impl Circle {
    fn new<T>(color: T) -> Self
    where
        T: Color + 'static,
    {
        Self {
            color: Box::new(color),
        }
    }
}
//Circle实现Shape
impl Shape for Circle {
    fn draw(&self) {
        println!("drawing a circle");
        self.color.apply_color();
    }
}
//Square【修正抽象（RefinedAbstraction）】
struct Square {
    color: Box<dyn Color>,
}
//Square实现Shape
impl Square {
    fn new<T>(color: T) -> Self
    where
        T: Color + 'static,
    {
        Self {
            color: Box::new(color),
        }
    }
}

impl Shape for Square {
    fn draw(&self) {
        println!("drawing a square");
        self.color.apply_color();
    }
}

#[test]
fn main() {
    let red = Red {};
    let circle = Circle::new(red);
    circle.draw();

    let blue = Blue {};
    let square = Square::new(blue);
    square.draw();
}
