use std::rc::Rc;

//Command命令【命令（Command）】
trait Command {
    fn execute(&self);
}

//LightOnCommand开灯命令【具体命令（Concrete Command）】
struct LightOnCommand {
    light: Rc<Light> ,
}

impl<'a> LightOnCommand {
    fn new( light: Rc<Light> ) -> Self {
        Self { light }
    }
}
//实现命令接口：LightOnCommand
impl<'a> Command for LightOnCommand {
    fn execute(&self) {
        self.light.turn_on();
    }
}

//LightOffCommand关灯命令【具体命令（Concrete Command）】
struct LightOffCommand {
    light: Rc<Light> ,
}

impl<'a> LightOffCommand{
    fn new(light: Rc<Light> ) -> Self {
        Self { light }
    }
}
//实现命令接口：LightOffCommand
impl<'a> Command for LightOffCommand {
    fn execute(&self) {
        self.light.turn_off();
    }
}

//Light灯【接收者（Receiver）】
struct Light {}
impl Light {
    fn turn_on(&self) {
        println!("Light is on");
    }

    fn turn_off(&self) {
        println!("Light is off");
    }
}

struct RemoteControl {
    command: Option<Box<dyn Command>>,
}
//RemoteControl【调用者/请求者（Invoker）】
impl RemoteControl {
    fn new() -> Self {
        Self { command: None }
    }
    fn set_command<T>(&mut self, command: T)
    where
        T: Command+'static 
    {
        self.command = Some(Box::new(command));
    }
}

impl RemoteControl {
    fn press_button(&self) {
        self.command.as_ref().map(|c|c.execute());
    }
}

#[test]
fn main() {

    //客户端（Client）
    //命令接收者
    let light = Rc::new(Light {});
    //开灯命令
    let light_on_command = LightOnCommand::new(light.clone());
    //关灯命令
    let light_off_command = LightOffCommand::new(light.clone());
    //命令调用者：远程控制1
    let mut remote_control = RemoteControl::new();
    //设置开灯
    remote_control.set_command(light_on_command);

    remote_control.press_button();
    //设置关烟命令
    remote_control.set_command(light_off_command);
    
    remote_control.press_button();
}
