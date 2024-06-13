//FileSystemComponent接口【组件（Component）】
trait FileSystemComponent {
    fn display_info(&self);
}

//File【叶子（Leaf）】
struct File {
    name: String,
}

impl File {
    fn new<T>(name: T) -> Self
    where
        T: Into<String>,
    {
        Self { name: name.into() }
    }
}
//File实现组件接口
impl FileSystemComponent for File {
    fn display_info(&self) {
        println!("File {}", self.name);
    }
}

//Directory【复合（Composite）】
struct Directory {
    name: String,
    components: Vec<Box<dyn FileSystemComponent>>,
}

impl Directory {
    fn new<T>(name: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            name: name.into(),
            components: Vec::new(),
        }
    }

    //添加组件
    fn add_component<T>(&mut self, component: T)
    where
        T: FileSystemComponent + 'static,
    {
        self.components.push(Box::new(component));
    }
}

//Directory实现组件接口
impl FileSystemComponent for Directory {
    fn display_info(&self) {
        println!("Directory:{}", self.name);
        for component in self.components.iter() {
            component.display_info();
        }
    }
}

#[test]
fn main() {
    let file1 = File::new("file1.txt");
    let file2 = File::new("file2.txt");

    let mut sub_directory = Directory::new("subDirectory");
    sub_directory.add_component(file1);
    sub_directory.add_component(file2);

    let mut root_directory = Directory::new("root");
    root_directory.add_component(sub_directory);
    root_directory.display_info();
}
