use std::collections::HashMap;



//主题：主题统一接口
trait Subject<'a> {
    fn add_observer(&mut self, observer: &'a mut dyn Observer);
    fn remove_observer(&mut self, id: &i32);
    fn notify_observers(&mut self);
}

//具体主题
struct ConcreteSubject<'a> {
    state: i32,
    observers: HashMap<i32, &'a mut dyn Observer>,
}

impl<'a> ConcreteSubject<'a> {
    fn new() -> Self {
        Self {
            state: 0,
            observers: HashMap::new(),
        }
    }
    fn set_state(&mut self,state:i32){
        self.state = state;
    }
}

//实现主题接口
impl <'a>Subject<'a> for ConcreteSubject<'a> {
    fn add_observer(&mut self, observer: &'a mut dyn Observer) {
        self.observers
            .insert(observer.get_id(), observer);
    }
    fn remove_observer(&mut self, id: &i32) {
        self.observers.remove(id);
    }
    fn notify_observers(&mut self) {
        for observer in self.observers.values_mut() {
            observer.update(self.state)
        }
    }
}

//观察者：观察者统一接口
trait Observer {
    fn get_id(&self) -> i32;
    fn update(&mut self, state: i32);
}

//具体观察者
struct ConcreteObserver {
    id: i32,
}

impl ConcreteObserver {
    fn new(id: i32) -> Self {
        Self {
            id,
        }
    }
}

//具体观察者实者观察者接口
impl Observer for ConcreteObserver {
    fn update(&mut self, state: i32) {
        println!("Observer : {} state:{}is updated", self.id, state);
    }
    fn get_id(&self) -> i32 {
        self.id
    }
}
#[test]
fn main(){
    let mut observer1 = ConcreteObserver::new(1);
    let mut observer2 = ConcreteObserver::new(2);
    let mut observer3 = ConcreteObserver::new(3);
    let mut subject = ConcreteSubject::new();
    subject.add_observer(&mut observer1);
    subject.add_observer(&mut observer2);
    subject.add_observer(&mut observer3);
    subject.set_state(100);
    subject.notify_observers();
    println!("remove observer2");
    subject.remove_observer(&2);

    subject.set_state(200);
    subject.notify_observers();
   
}