use crate::container::Container;

pub struct Basket<T> {
    items: Option<T>,
}

impl<T> Basket<T> {
    pub fn new(item: T) -> Self {
        Basket { items: Some(item) }
    }
}

impl<T> Container<T> for Basket<T> {
    fn get(&mut self) -> Option<T> {
        self.items.take()
    }

    fn put(&mut self, item: T) {
        self.items = Some(item);
    }

    fn is_empty(&self) -> bool {
        self.items.is_none()
    }
}
