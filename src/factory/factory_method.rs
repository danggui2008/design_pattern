//图形接口【抽象产品（Abstract Product）】
trait Shape {
    fn draw(&self);
}
//圆【具体产品（Concrete Product）】
struct Circle{}

//Circle实现抽象产品
impl Shape for Circle {
    fn draw(&self) {
        println!("drawing a circle");
    }
}
//矩形【具体产品（Concrete Product）】
struct Rectangle;

//Rectangle实现抽象产品
impl Shape for Rectangle {
    fn draw(&self) {
        println!("drawing a  rectangle")
    }
}

//图形工厂抽象接口【抽象工厂（Abstract Factory）】
trait ShapeFactory {
    fn create_shape(&self) -> Box<dyn Shape>;
}


//圆工厂：负责创建圆形产品【具体工厂（Concrete Factory）】
struct CircleFactory;

impl ShapeFactory for CircleFactory {
    fn create_shape(&self) -> Box<dyn Shape> {
        Box::new(Circle{})
    }
}

//矩形工厂：负责创建矩形产品【具体工厂（Concrete Factory）】
struct RectangleFactory;

impl ShapeFactory for RectangleFactory {
    fn create_shape(&self) -> Box<dyn Shape> {
        Box::new(Rectangle)
    }
}

#[test]
fn main() {
    /*
    CircleFactory只负责创建圆，RectangleFactory负责创建矩形，
    每种产品都有相应的工厂来创建，在新增产品时，只需要新增相应
    的工厂就可，做到了“开闭原则”。
     */
    let factory1 = CircleFactory;
    let factory2 = RectangleFactory;
    let circle = factory1.create_shape();
    circle.draw();
    let rectangle = factory2.create_shape();
    rectangle.draw();
}
