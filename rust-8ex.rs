use std::cmp::Ordering;

// Универсальная функция поиска минимума
// Работает с любым типом, который можно сравнивать (Ord)
fn min<T: Ord>(a: T, b: T) -> T {
    match a.cmp(&b) {
        Ordering::Less => a,
        Ordering::Equal => a,
        Ordering::Greater => b,
    }
}

fn main() {
    println!("{}", min(0, 10));
    println!("{}", min(500, 123));

    println!("{}", min('a', 'z'));
    println!("{}", min("hello", "goodbye"));
}