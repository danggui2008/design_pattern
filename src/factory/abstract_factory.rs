//操作系统【抽象产品（Abstract Product）】
trait OperatingSystem {
    fn run(&self);
}

//Windows操作系统【具体产品（Concrete Product）】
struct WindowsOS;
impl OperatingSystem for WindowsOS {
    fn run(&self) {
        println!("running in windows os ")
    }
}

//Linux操作系统【具体产品（Concrete Product）】
struct LinuxOS;

impl OperatingSystem for LinuxOS {
    fn run(&self) {
        println!("running in linux os")
    }
}

//应用程序【抽象产品（Abstract Product）】
trait Application {
    fn open(&self);
}

//word应用程序【具体产品（Concrete Product）】
struct WordApplication;

impl Application for WordApplication {
    fn open(&self) {
        println!("打开word应用程序")
    }
}

//excel应用程序【具体产品（Concrete Product）】
struct ExcelApplication;

impl Application for ExcelApplication {
    fn open(&self) {
        println!("打开Excel应用程序")
    }
}

//软件工厂【抽象工厂（Abstract Factory）】
trait SoftwareFactory {
    fn create_operating_system(&self) -> Box<dyn OperatingSystem>;
    fn create_application(&self) -> Box<dyn Application>;
}

//Window工厂【具体工厂（Concrete Factory）】
struct WindowsSoftwareFactory;

impl SoftwareFactory for WindowsSoftwareFactory {
    fn create_operating_system(&self) -> Box<dyn OperatingSystem> {
        Box::new(WindowsOS)
    }
    fn create_application(&self) -> Box<dyn Application> {
        Box::new(WordApplication)
    }
}

//Linux工厂【具体工厂（Concrete Factory）】
struct LinuxSoftwareFactory;

impl SoftwareFactory for LinuxSoftwareFactory {
    fn create_operating_system(&self) -> Box<dyn OperatingSystem> {
        Box::new(LinuxOS)
    }

    fn create_application(&self) -> Box<dyn Application> {
        Box::new(ExcelApplication)
    }
}

#[test]
fn main() {
    let windows_factory = WindowsSoftwareFactory;
    let windows_os = windows_factory.create_operating_system();
    let windows_app = windows_factory.create_application();
    windows_os.run();
    windows_app.open();

    let linux_factory = LinuxSoftwareFactory;
    let linux_os = linux_factory.create_operating_system();
    let linux_app = linux_factory.create_application();
    linux_os.run();
    linux_app.open();
}
