/*
这里可以不使用Rc,使用借用
*/

//Command命令【命令（Command）】
trait Command {
    fn execute(&self);
}

//LightOnCommand开灯命令【具体命令（Concrete Command）】
struct LightOnCommand<'l> {
    light: &'l Light,
}

impl<'l> LightOnCommand<'l> {
    fn new(light: &'l Light) -> Self {
        Self { light }
    }
}
//实现命令接口：LightOnCommand
impl<'l> Command for LightOnCommand<'l> {
    fn execute(&self) {
        self.light.turn_on();
    }
}

//LightOffCommand关灯命令【具体命令（Concrete Command）】
struct LightOffCommand<'l> {
    light: &'l Light,
}

impl<'l> LightOffCommand<'l> {
    fn new(light: &'l Light) -> Self {
        Self { light }
    }
}
//实现命令接口：LightOffCommand
impl<'l> Command for LightOffCommand<'l> {
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

struct RemoteControl<'l> {
    command: Option<Box<dyn Command + 'l>>,
}
//RemoteControl【调用者/请求者（Invoker）】
impl<'l> RemoteControl<'l> {
    fn new() -> Self {
        Self { command: None }
    }
    fn set_command<T>(&mut self, command: T)
    where
        T: Command + 'l,
    {
        self.command = Some(Box::new(command));
    }
}

impl<'l> RemoteControl<'l> {
    fn press_button(&self) {
        self.command.as_ref().map(|c| c.execute());
    }
}

#[test]
fn main() {
    //客户端（Client）
    //命令接收者
    let light = Light {};
    //开灯命令
    let light_on_command = LightOnCommand::new(&light);
    //关灯命令
    let light_off_command = LightOffCommand::new(&light);
    //命令调用者：远程控制1
    let mut remote_control = RemoteControl::new();
    //设置开灯
    remote_control.set_command(light_on_command);

    remote_control.press_button();
    //设置关烟命令
    remote_control.set_command(light_off_command);

    remote_control.press_button();
}
