use std::{sync::Mutex, thread};
use lazy_static::lazy_static;
/*
通过lazy_static实现单例模式
*/
#[derive(Debug)]
pub struct Singleton {
    element: Vec<i32>,
}

lazy_static!{
    static ref INSTANCE:Mutex<Singleton> = Mutex::new(Singleton{
        element: Vec::new(),
    });
}

//显示element地址
fn info(){
    println!("Singleton element addr:{:p}",&INSTANCE.lock().unwrap().element);
}

#[test]
fn main() {
    let handler1 = thread::spawn(|| {
        info();
    });
    let handler2 = thread::spawn(|| {
        info();
    });
    handler1.join().unwrap();
    handler2.join().unwrap();
}
