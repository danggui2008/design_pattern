//图形接口【抽象产品（Abstract Product）】
trait Shape {
    fn draw(&self);
}
//圆【具体产品（Concrete Product）】
struct Circle;

//实现产品接口
impl Shape for Circle {
    fn draw(&self) {
        println!("drawing a circle");
    }
}
//矩形【具体产品（Concrete Product）】
struct Rectangle;

//实现产品接口
impl Shape for Rectangle {
    fn draw(&self) {
        println!("drawing a  rectangle")
    }
}

//工厂【简单工厂】
struct SimpleFactory;

impl SimpleFactory {
    //根据产品名称创建不同的产品
    fn create_shape(name: &str) -> Option<Box<dyn Shape>> {
        if name == "Circle" {
            return Some(Box::new(Circle));
        } else if name == "Rectangle" {
            return Some(Box::new(Rectangle));
        } else {
            None
        }
    }
}

#[test]
fn main() {
    //客户端根据型业务通过工厂SimpleFactory创建不同的产品
    let circle = SimpleFactory::create_shape("Circle");
    if let Some(circle) = circle {
        circle.draw();
    }

    let rectangle = SimpleFactory::create_shape("Rectangle");
    if let Some(rectangle) = rectangle {
        rectangle.draw();
    }
    //create other
    let other = SimpleFactory::create_shape("other");
    if other.is_some() {
        other.unwrap().draw();
    } else {
        println!("没有产品");
    }
}
