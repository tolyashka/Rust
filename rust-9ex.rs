use std::collections::HashMap;
use std::hash::Hash;

// Структура для подсчёта количества элементов
struct Counter<T> {
    values: HashMap<T, u64>,
}

impl<T: Eq + Hash> Counter<T> {
    fn new() -> Self {
        Counter {
            values: HashMap::new(),
        }
    }

    // Увеличивает счётчик значения
    fn count(&mut self, value: T) {
        if self.values.contains_key(&value) {
            *self.values.get_mut(&value).unwrap() += 1;
        } else {
            self.values.insert(value, 1);
        }
    }

    // Возвращает количество появлений
    fn times_seen(&self, value: &T) -> u64 {
        self.values.get(value).copied().unwrap_or(0)
    }
}

fn main() {
    let mut ctr = Counter::new();

    ctr.count(10);
    ctr.count(10);
    ctr.count(5);

    println!("10 -> {}", ctr.times_seen(&10));
    println!("5 -> {}", ctr.times_seen(&5));
}