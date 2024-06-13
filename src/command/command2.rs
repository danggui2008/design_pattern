use std::rc::Rc;

//视频播放器执行命令【抽象命令】
trait IAction {
    //执行命令
    fn execute(&self);
}

//视频播放器-关闭播放命令【具体命令】
struct CloseAction {
    player: Rc<Player>,
}
impl CloseAction {
    fn new(player: Rc<Player>) -> Self {
        Self { player }
    }
}
impl IAction for CloseAction {
    fn execute(&self) {
        self.player.close();
    }
}

//视频播放器-暂停播放命令【具体命令】
struct PauseAction {
    player: Rc<Player>,
}
impl PauseAction {
    fn new(player: Rc<Player>) -> Self {
        Self { player }
    }
}

impl IAction for PauseAction {
    fn execute(&self) {
        self.player.pause();
    }
}

//视频播放器-加速播放命令【具体命令】
struct SpeedAction {
    player: Rc<Player>,
}
impl SpeedAction {
    fn new(player: Rc<Player>) -> Self {
        Self { player }
    }
}
impl IAction for SpeedAction {
    fn execute(&self) {
        self.player.speed();
    }
}

//视频播放器-停止播放命令【具体命令】
struct StopAction {
    player: Rc<Player>,
}
impl StopAction {
    fn new(player: Rc<Player>) -> Self {
        Self { player }
    }
}
impl IAction for StopAction {
    fn execute(&self) {
        self.player.stop();
    }
}

//视频播放器【命令接收者】
struct Player {}

impl Player {
    fn close(&self) {
        println!("视频播放关闭...")
    }
    fn pause(&self) {
        println!("视频暂停播放...");
    }
    fn speed(&self) {
        println!("视频加速播放...");
    }
    fn stop(&self) {
        println!("视频停止播放...");
    }
}

//视频播放控制器命令执行者
struct Controller {
    action: Box<dyn IAction>,
}

impl Controller {
    fn new<T>(action: T) -> Self
    where
        T: IAction + 'static,
    {
        Self {
            action: Box::new(action),
        }
    }

    fn execute(&self) {
        self.action.execute();
    }
}

#[test]
fn main() {
    let player = Rc::new(Player {});
    let speed_action = SpeedAction::new(player.clone());
    let close_action = CloseAction::new(player.clone());
    let controller1 = Controller::new(speed_action);
    let controller2 = Controller::new(close_action);
    controller1.execute();
    controller2.execute();
}
