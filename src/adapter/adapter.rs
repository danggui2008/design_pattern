//LegacyRectangle【源角色（Adaptee】
struct LegacyRectangle {}

impl LegacyRectangle {
    //原有接口
    fn display(&self, x1: i32, y1: i32, x2: i32, y2: i32) {
        println!("LegacyRectangle({},{},{},{})", x1, y1, x2, y2);
    }
}

//Shape接口【目标角色（Target）】
trait Shape {
    fn draw(&self, x: i32, y: i32, width: i32, height: i32);
}

//RectangleAdapter【适配器（Adapter）】
struct RectangleAdapter {
    rectangle: LegacyRectangle,
}
impl RectangleAdapter {
    fn new(rectangle: LegacyRectangle) -> Self {
        Self { rectangle }
    }
}

//RectangleAdapter实现Shape接口
impl Shape for RectangleAdapter {

    fn draw(&self, x: i32, y: i32, width: i32, height: i32) {
        let x2 = x + width;
        let y2 = y + height;
        //适配display
        self.rectangle.display(x, y, x2, y2);
    }
}

#[test]
fn main(){
    let rectangle = LegacyRectangle{};
    //适配rectangle
    let shape_adapter = RectangleAdapter::new(rectangle);
    shape_adapter.draw(10, 10, 100, 50);
}