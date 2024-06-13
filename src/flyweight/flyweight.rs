use std::collections::HashMap;

//Shape【抽象享元角色（Flyweight）】
trait Shape {
    fn draw(&self, coordinate: Coordinate);
}

//Color颜色【内部状态】
struct Color(&'static str);
//Coordinate坐标【外部状态】
struct Coordinate {
    x: f32,
    y: f32,
}
impl Coordinate {
    fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

/*
Circle【具体享元角色（Concrete Flyweight）】
圆：画一个圆包括：坐标，颜色
这里我们假设颜色只有：红、绿、蓝，黄，而坐标不固定。
那么圆的坐标适合作为对象的外部状态，而颜色为：内部状态。
*/
struct Circle {
    color: Color,
}

impl<'c> Circle {
    fn new(color: Color) -> Self {
        Self { color }
    }
}

impl<'c> Shape for Circle {
    fn draw(&self, coordinate: Coordinate) {
        println!(
            "Draw a {} color circle at x:{},y:{}",
            &self.color.0, coordinate.x, coordinate.y,
        )
    }
}

//ShapeFactory【元工厂（Flyweight Factory）】
struct ShapeFactory {
    //缓存内部对象：颜色，颜色作为Shape（这里是圆的对象内部状态）
    shapes: HashMap<String, Box<dyn Shape>>,
}

impl ShapeFactory {
    fn new() -> Self {
        Self {
            shapes: HashMap::new(),
        }
    }

    fn get_shape(&mut self, color: &'static str) -> Option<&Box<dyn Shape>> {
        if !self.shapes.contains_key(color) {
            self.shapes
                .insert(color.to_string(), Box::new(Circle::new(Color(color))));
        }
        return self.shapes.get(color);
    }
}
#[test]
fn main() {
    let mut factory = ShapeFactory::new();

    let colors = ["red", "green", "blue", "yellow"];
    for i in 0..10 {
        for c in 0..4 {
            let x = (i + c + 10) as f32;
            let y = (i + c + 20) as f32;
            //对象外部状态：坐标
            let coordinate = Coordinate::new(x, y);
            let shape = factory.get_shape(colors[c]);
            shape.map(|shape| shape.draw(coordinate));
        }
    }
}
