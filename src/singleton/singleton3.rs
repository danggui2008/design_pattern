use std::{
    sync::{Arc, Once},
    thread,
};
/*
通过Once实现单例模式
*/
#[derive(Debug)]
pub struct Singleton {
    element: Vec<i32>,
}

static mut INSTANCE: Option<Arc<Singleton>> = None;
static INIT: Once = Once::new();

impl Singleton {
    pub fn get_singleton() -> Arc<Singleton> {
        INIT.call_once(|| unsafe {
            INSTANCE = Some(Arc::new(Self {
                element: Vec::new(),
            }));
        });
        unsafe { INSTANCE.clone().unwrap() }
    }
    fn info(&self) {
        println!("Singleton elements addr:{:p}", &self.element);
    }
}

#[test]
fn main() {
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
