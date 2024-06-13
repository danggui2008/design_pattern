use std::{
    cell::RefCell,
    collections::HashMap,
    rc::{Rc, Weak},
};
//ChatMediator【Mediator（抽象中介者）】
trait ChatMediator {
    //通知接口：同事通过此接口与同事通信
    fn notify(&mut self, name: &str, message: &str);
    fn register(&mut self, colleague: Weak<(dyn Colleague)>);
}
//ConcreteChatMediator【ConcreteMediator（具体中介者）】
struct ConcreteChatMediator {
    colleagues: HashMap<String, Weak<dyn Colleague>>,
}

impl ConcreteChatMediator {
    fn new() -> Self {
        Self {
            colleagues: HashMap::new(),
        }
    }
}

impl ChatMediator for ConcreteChatMediator {
    fn notify(&mut self, name: &str, message: &str) {
        //用户通知中介者，中价根据业务情况，完成相应的业务，这里调用同事的action方法完成业务
        if let Some(colleague) = self.colleagues.get(name) {
            if let Some(colleague) = colleague.upgrade() {
                colleague.action(message);
            }
        }
    }
    fn register(&mut self, colleague: Weak<dyn Colleague>) {
        if let Some(weak_colleague) = colleague.upgrade() {
            self.colleagues
                .insert(weak_colleague.get_name().to_string(), colleague);
        }
    }
}
//Colleague【Colleague（抽象同事类）】
trait Colleague {
    fn get_name(&self) -> &str;
    //同事行动起来：中介者与之通信
    fn action(&self, message: &str);
    //发送消息：通过中介者与与具体同事通信
    fn send_message(&self, message: &str);
}

//ConcreteColleague【ConcreteColleague（具体同事类）】
struct ConcreteColleague {
    name: String,
    mediator: Rc<RefCell<dyn ChatMediator>>,
}

impl ConcreteColleague {
    fn new<T>(name: T, mediator: Rc<RefCell<dyn ChatMediator>>) -> Self
    where
        T: Into<String>,
    {
        Self {
            name: name.into(),
            mediator,
        }
    }
}

impl<'c> Colleague for ConcreteColleague {
    fn get_name(&self) -> &str {
        &self.name
    }
    fn action(&self, message: &str) {
        println!("[{}]收到消息:{}，我马上行动起来", self.name, message);
    }
    fn send_message(&self, message: &str) {
        println!("将要给[{}]发送消息:{}", self.name, message);
        self.mediator.borrow_mut().notify(&self.name, message);
    }
}

#[test]
fn main() {
    let mediator = Rc::new(RefCell::new(ConcreteChatMediator::new()));
    let colleague1 = Rc::new(ConcreteColleague::new("码海悔道", mediator.clone()));
    let colleague2 = Rc::new(ConcreteColleague::new("小玉", mediator.clone()));
    let colleague3 = Rc::new(ConcreteColleague::new("小李", mediator.clone()));
    {
        //这里要注意
        let mut mediator = mediator.borrow_mut();
        mediator.register(Rc::downgrade(&colleague1) as Weak<dyn Colleague>);
      
        mediator.register(Rc::downgrade(&colleague2) as Weak<dyn Colleague>);
        
        mediator.register(Rc::downgrade(&colleague3) as Weak<dyn Colleague>);
    }

    colleague1.send_message("月色真好");
    colleague2.send_message("来了");
}
