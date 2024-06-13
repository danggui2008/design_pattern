//Shape接口具体原型（Concrete Prototype）
#[derive(Debug)]
struct Shape {
    s_type: String,
}
impl Shape {
    fn new<T>(s_type: T) -> Self 
    where
    T:Into<String>{
        Self {
            s_type: s_type.into(),
        }
    }
}
//实现：Clone trait【抽象原型（Prototype）】
impl Clone for Shape {
    fn clone(&self) -> Self {
        Self {
            s_type: self.s_type.clone(),
        }
    }
}

#[test]
fn main() {
    //客户端（Client）
    let shape1 = Shape::new("s_type1");
    println!("shape1:{:#?}", shape1);
    //通过clone来创建对象
    let shape2 = shape1.clone();
    println!("shape2:{:#?}", shape2);
}
