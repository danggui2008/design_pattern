//Memento【Memento（备忘录）】
#[derive(Debug, Clone)]
struct Memento {
    state: String,
}

impl Memento {
    fn new<T>(state: T) -> Self 
    where
    T:Into<String>{
        Self {
            state: state.into(),
        }
    }
    //获取状态
    fn get_state(self) -> String {
        self.state
    }
}

// Originator【Originator（发起人】
#[derive(Debug)]
struct Originator {
    state: String,
}

impl Originator {
    fn new(state: &str) -> Self {
        Self {
            state: state.to_string(),
        }
    }
}

impl Originator {
    fn set_state(&mut self, state: &str) {
        self.state = state.to_string();
    }
    //以当前状态创建备忘录
    fn create_memento(&self) -> Memento {
        Memento::new(&self.state)
    }
    //根据备忘录来恢复状态
    fn restore_memento(&mut self, memento: Memento) {
        self.state = memento.get_state();
    }
}

//Caretaker【Caretaker（负责人）】
struct Caretaker {
    mementos: Vec<Memento>,
}

impl Caretaker {
    fn new() -> Self {
        Self {
            mementos: Vec::new(),
        }
    }
}

impl Caretaker {
    //添加备忘录
    fn add_memento(&mut self, memento: Memento) {
        self.mementos.push(memento);
    }

    //获取备忘录
    fn get_memento(&self, index: usize) -> Option<Memento> {
        self.mementos.get(index).map_or(None, |m| Some(m.clone()))
    }
}
#[test]
fn main(){
    //发起人
    let mut originator = Originator::new("state 1");
    //创建备注录：state 1
    let memento =  originator.create_memento();
    //负责人
    let mut caretaker = Caretaker::new();
    caretaker.add_memento(memento);
    originator.set_state("state 2");
    //state 2
    println!("{:#?}",originator);
    //恢复状态(state 2-》state 1)
    originator.restore_memento(caretaker.get_memento(0).unwrap());
     //已恢复到原来状态：state 1
    println!("{:#?}",originator);
}
