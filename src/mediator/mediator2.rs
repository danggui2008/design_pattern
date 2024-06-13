use std::collections::HashMap;

trait ChatMediator {
    //通知接口：同事通过此接口与同事通信
    fn notify(&mut self, name: &str, message: &str);
    fn register(&mut self, colleague: Box<dyn Colleague>);
}
struct ConcreteChatMediator {
    colleagues: HashMap<String, Box<dyn Colleague>>,
}

impl ConcreteChatMediator {
    fn new() -> Self {
        Self {
            colleagues: HashMap::new(),
        }
    }
    fn send_message(&mut self, name: &str, message: &str) {
        //这样不行
        //closure requires unique access to `*self` but it is already borrowed
        //self.colleagues.get_mut(name).map(|c|c.send_message(self, message));\
        //self.colleagues.get_key_value(k)
        if let Some(mut colleague) = self.colleagues.remove(name) {
            colleague.send_message(self, message);
            //临时这样
            self.colleagues.insert(name.to_string(), colleague);
        }
    }
}

impl ChatMediator for ConcreteChatMediator {
    fn notify(&mut self, name: &str, message: &str) {
        //用户通知中介者，中价根据业务情况，完成相应的业务，这里调用同事的action方法完成业务
        if let Some(colleague) = self.colleagues.get_mut(name) {
            colleague.action(message);
        }
    }
    fn register(&mut self, colleague: Box<dyn Colleague>) {
        self.colleagues
            .insert(colleague.get_name().to_string(), colleague);
    }
}
//抽象同事：
trait Colleague {
    fn get_name(&self) -> &str;
    //同事行动起来：中介者与之通信
    fn action(&mut self, message: &str);
    //发送消息：通过中介者与与具体同事通信
    fn send_message(&mut self, mediator: &mut dyn ChatMediator, message: &str);
}

struct ConcreteColleague {
    name: String,
}

impl ConcreteColleague {
    fn new<T>(name: T) -> Self
    where
        T: Into<String>,
    {
        Self { name: name.into() }
    }
}

impl Colleague for ConcreteColleague {
    fn get_name(&self) -> &str {
        &self.name
    }
    fn action(&mut self, message: &str) {
        println!("[{}]收到消息:{}，我马上行动起来", self.name, message);
    }
    fn send_message(&mut self, mediator: &mut dyn ChatMediator, message: &str) {
        println!("将要给[{}]发送消息:{}", self.name, message);
        mediator.notify(&self.name, message);
    }
}

#[test]
fn main() {
    let mut mediator = ConcreteChatMediator::new();
    let colleague1 = ConcreteColleague::new("码海悔道");
    let colleague2 = ConcreteColleague::new("小玉");
    let colleague3 = ConcreteColleague::new("小李");

    mediator.register(Box::new(colleague1));
    mediator.register(Box::new(colleague2));
    mediator.register(Box::new(colleague3));
    mediator.send_message("码海悔道", "月色真好");
    mediator.send_message("码海悔道", "月色真好");
    mediator.send_message("小玉", "来了");
}
