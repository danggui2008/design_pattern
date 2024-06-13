//StereoSystem音响【子系统角色（SubSystem）】
struct StereoSystem {}

impl StereoSystem {
    fn turn_on(&self) {
        println!("打开音响")
    }

    fn turn_off(&self) {
        println!("关闭音响")
    }
}
// Projector投影仪【子系统角色（SubSystem）】
struct Projector {}

impl Projector {
    fn turn_on(&self) {
        println!("打开投影仪")
    }

    fn turn_off(&self) {
        println!("关闭投影仪")
    }
}

// LightsControl灯光控制【子系统角色（SubSystem）】
struct LightsControl {}

impl LightsControl {
    fn turn_on(&self) {
        println!("灯光已打开")
    }

    fn turn_off(&self) {
        println!("灯光关闭")
    }
}
// HomeTheaterFacade家庭影院外观【外观角色（Facade）】
struct HomeTheaterFacade {
    stereo: StereoSystem,
    projector: Projector,
    lights: LightsControl,
}

impl HomeTheaterFacade {
    fn new(stereo: StereoSystem, projector: Projector, lights: LightsControl) -> Self {
        Self {
            stereo,
            projector,
            lights,
        }
    }
}

impl HomeTheaterFacade {
    fn watch_movie(&self) {
        println!("开始看电影");
        self.lights.turn_off();
        self.projector.turn_off();
        self.stereo.turn_on();
    }

    fn end_move(&self) {
        println!("电影结束");
        self.stereo.turn_off();
        self.projector.turn_off();
        self.lights.turn_on();
    }
}

#[test]
fn main() {
    let home_theater = HomeTheaterFacade::new(StereoSystem {}, Projector {}, LightsControl {});
    //开始看电影
    home_theater.watch_movie();
    //观影结束
    home_theater.end_move();
}
