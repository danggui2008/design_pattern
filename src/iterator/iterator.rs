/*
rust中迭代器模式其实不是这样实现的。。。
*/
trait Iterator {
    type Item;
    fn has_next(&self) -> bool;
    fn next(&mut self) -> Option<Self::Item>;
}

struct ConcreteIterator<T> {
    items: Vec<T>,
    position: usize,
}
impl<T> ConcreteIterator<T> {
    fn new(items: Vec<T>) -> Self {
        Self {
            position: items.len(),
            items,
        }
    }
}

impl<T> Iterator for ConcreteIterator<T> {
    type Item = T;
    fn has_next(&self) -> bool {
        return self.position > 0;
    }
    fn next(&mut self) -> Option<Self::Item> {
        if self.has_next() {
            self.position -= 1;
            return self.items.pop();
        } else {
            return None;
        }
    }
}

trait IterableCollection<T> {
    fn create_iterator(self) -> Box<dyn Iterator<Item = T>>;
}

struct ConcreteCollection<T> {
    items: Vec<T>,
}

impl<T> ConcreteCollection<T> {
    fn new() -> Self {
        Self { items: Vec::new() }
    }
}
impl<T> ConcreteCollection<T> {
    fn add(&mut self, item: T) {
        self.items.push(item);
    }
}

impl<T: 'static> IterableCollection<T> for ConcreteCollection<T> {
    fn create_iterator(self) -> Box<dyn Iterator<Item = T>> {
        Box::new(ConcreteIterator::new(self.items))
    }
}

#[test]
fn main() {
    let mut collection = ConcreteCollection::<String>::new();
    collection.add("A".to_string());
    collection.add("B".to_string());
    collection.add("C".to_string());
    
    
    let mut iterator = collection.create_iterator();
    while iterator.has_next() {
        println!("item:{}", iterator.next().unwrap());
    }
}
