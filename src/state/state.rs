//ElevatorState【抽象状态角色（State）】
trait ElevatorState {
    fn open_door(&self);
    fn close_door(&self);
    fn e_move(&self);
    fn e_stop(&self);
}

//ElevatorState电梯开门状态【具体状态角色（ConcreteState）】
struct OpenState {}

impl ElevatorState for OpenState {
    fn open_door(&self) {
        println!("door is already opened");
    }
    fn close_door(&self) {
        println!("closing the door");
    }
    fn e_move(&self) {
        println!("cannot move while is the door is opened")
    }
    fn e_stop(&self) {
        println!("is stopped");
    }
}

//CloseState电梯关门状态【具体状态角色（ConcreteState）】
struct CloseState {}

impl ElevatorState for CloseState {
    fn open_door(&self) {
        println!("opening the door");

    }
    fn close_door(&self) {
        println!("the door is closed");
    }
    fn e_move(&self) {
        println!("moving")
    }
    fn e_stop(&self) {
        println!("stopping");
    }
}

//Elevator电梯【环境角色（Context）】
struct Elevator {
    state: Box<dyn ElevatorState>,
}

impl Elevator {
    fn new() -> Self {
        Self {
            state: Box::new(CloseState {}),
        }
    }
}

impl Elevator {
    fn set_state<T>(&mut self, state: T)
    where
        T: ElevatorState + 'static,
    {
        self.state = Box::new(state);
    }
    fn open_door(&mut self) {
        self.state.open_door();
        self.state = Box::new(OpenState{});
    }
    fn close_door(&mut self) {
        self.state.close_door();
        self.state = Box::new(CloseState{});
    }
    fn e_move(&self) {
        self.state.e_move();
    }
    fn e_stop(&self) {
        self.state.e_stop();
    }
}

#[test]
fn main() {
    let mut elevator = Elevator::new();
    //设置电梯为开门状态
    elevator.set_state(OpenState{});
    elevator.open_door();//开门
    elevator.e_move();//开门无法移动
    elevator.close_door();//关门
    elevator.e_move();//移动
    elevator.e_stop();//停止
    elevator.open_door();//开门
}
