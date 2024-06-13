use std::f32::consts::PI;

//图形形状接口:accept接受访问者【Element类】
trait Shape {
    fn accept(&self, visitor: &mut dyn ShapeVisitor);
}
//Circle【ConcreteElement类】
struct Circle {
    radius: f32,
}
//圆：实现accept
impl Circle {
    fn new(radius: f32) -> Self {
        Self { radius }
    }
}

impl Shape for Circle {
    fn accept(&self, visitor: &mut dyn ShapeVisitor) {
        visitor.visit_circle(&self);
    }
}

//Rectangle【ConcreteElement类】
struct Rectangle {
    width: f32,
    height: f32,
}
//矩形：实现accept
impl Rectangle {
    fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }
}

impl Shape for Rectangle {
    fn accept(&self, visitor: &mut dyn ShapeVisitor) {
        visitor.visit_rectangle(self);
    }
}

//图形访问者接口【抽象Visitor】
trait ShapeVisitor {
    fn visit_circle(&mut self, circle: &Circle);
    fn visit_rectangle(&mut self, rectangle: &Rectangle);
}

//AreaCalculator【Visitor类】
struct AreaCalculator {
    area: f32,
}

//面积：实现访问者接口，计算不同形状的面积之和
impl AreaCalculator {
    fn new() -> Self {
        Self { area: 0.00 }
    }
    fn get_area(&self) -> f32 {
        self.area
    }
}

impl ShapeVisitor for AreaCalculator {
    //计算圆面积
    fn visit_circle(&mut self, circle: &Circle) {
        self.area += PI * circle.radius * circle.radius;
    }
    //计算矩形面积
    fn visit_rectangle(&mut self, rectangle: &Rectangle) {
        self.area += rectangle.width * rectangle.height;
    }
}
#[test]
pub fn main() {
    let circle = Circle::new(10.00);
    let rectangle = Rectangle::new(10.00, 12.00);

    let mut area_calculator = AreaCalculator::new();
    circle.accept(&mut area_calculator);
    let area = area_calculator.get_area();
    println!("circle area:{}", area);
    rectangle.accept(&mut area_calculator);
    let area = area_calculator.get_area();
    println!("total area:{}", area);
}
