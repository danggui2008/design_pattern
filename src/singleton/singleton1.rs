
use std::{
    sync::{Arc, Mutex}, thread,
};
/*
通过全局锁方式创建单例

*/
#[derive(Debug)]
struct Singleton {
    element: Vec<i32>,
}

static mut INSTANCE: Option<Arc<Singleton>> = None;
static LOCK: Mutex<()> = Mutex::new(());

impl Singleton {
    pub fn get_singleton() -> Arc<Singleton> {
        unsafe {
            if INSTANCE.is_none() {
                let lock = LOCK.lock().unwrap();
                INSTANCE.get_or_insert_with(|| {
                    Arc::new(Self {
                        element: Vec::new(),
                    })
                });
                INSTANCE.clone().unwrap()
            } else {
                INSTANCE.clone().unwrap()
            }
        }
    }
    //测试辅助方法
    fn info(&self) {
        println!("Singleton elements addr:{:p}", &self.element);
    }
}

#[test]
fn main() {
     //多线程方式获取单实例
    let handler1 = thread::spawn(|| {
        let instance = Singleton::get_singleton();
        instance.info();
    });
    let handler2 = thread::spawn(|| {
        let instance = Singleton::get_singleton();
        instance.info();
    });
    handler1.join().unwrap();
    handler2.join().unwrap();
}
