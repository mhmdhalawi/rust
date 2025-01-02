mod basket;
mod stack;
mod container;

use basket::Basket;
use container::Container;
use stack::Stack;

fn add_string<T: Container<String>>(c: &mut T, s: String) {
    c.put(s);
}

fn main() {
    let mut b1 = Basket::new("apple".to_string());
    b1.put("banana".to_string());
    b1.get();
    b1.is_empty();

    let mut s1 = Stack::new(vec![1, 2, 3]);
    s1.put(4);
    s1.get();
    s1.is_empty();

    let mut s2 = Stack::new(vec!["a".to_string(), "b".to_string(), "c".to_string()]);

    add_string(&mut b1, "orange".to_string());
    add_string(&mut s2, "d".to_string());
}
