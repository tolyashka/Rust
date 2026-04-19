// Задание 21. Обедающие философы
// • Пять философов обедают за одним столом.
// • У каждого за столом свое место.
// • Между каждой тарелкой лежит одна вилка.
// • Блюдо, которое они едят, можно есть только двумя вилками.
// • Каждый философ может одновременно или думать иди есть.
//• Но есть он может, только когда две вилки рядом с ним
// свободны, то есть оба его соседа думают, а не едят.
// • Каждый философ поев, кладет свои вилки.

use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

struct Fork;
// Обертка Mutex в Arc нужна для того чтобы можно было получить
// доступ к мьютексу из нескольких потоков
// Иначе компилятор не сможет передать его внутрь замыкания потока

// ARC = умный указатель с атомарным счётчиком ссылок
// Arc позволяет нескольким потокам владеть одним и тем же объектом.
// Arc НЕ делает данные потокобезопасными - гонка потоков есть 

struct Philosopher {
    name: String,
    left_fork: Arc<Mutex<Fork>>, // умный указатель с разделяемым владением
    right_fork: Arc<Mutex<Fork>>,
    thoughts: mpsc::Sender<String>, // много отправителей - один получатель. 
    // канал передачи сообщенйи между потоками
    // пять философов, каждый в сыоём потоке, передают сообщения в главный поток main
}

impl Philosopher {
    fn think(&self) {
        self.thoughts
            // send возвращает Result<(), SendError<T>>
            .send(format!("Eureka! {} has a new idea!", &self.name))
            .unwrap(); // автоматически матчит Result и если error - упадет
        thread::sleep(Duration::from_millis(1));
    }

    fn eat(&self) {
        // Взять вилки ...
        if self.name == "Пифагор" {
            // Нужны 2 вилки, чтобы есть
            let _right = self.right_fork.lock().unwrap();
            let _left = self.left_fork.lock().unwrap();
            println!("{} ест ...", &self.name);
            thread::sleep(Duration::from_millis(10));
        } else {
            // Нужны 2 вилки, чтобы есть
            let _left = self.left_fork.lock().unwrap();
            let _right = self.right_fork.lock().unwrap();
            println!("{} ест ...", &self.name);
            thread::sleep(Duration::from_millis(10));
        }
        // нам не нужно самостоятельно высвобождать блокировки мьютексов, 
        // ибо это делается автоматически при выходе из области видимости
    }
}

static PHILOSOPHERS: &[&str] =
    &["Сократ", "Гипатия", "Платон", "Aристотель", "Пифагор"];

fn main() {
    // создаём канал
    let (tx, rx) = mpsc::channel();
    // Создать вилки
    let forks = (0..5).map(|_| Arc::new(Mutex::new(Fork))).collect::<Vec<_>>();
    
    let philo = PHILOSOPHERS.iter().enumerate().map(|(i, name)| 
        Philosopher { 
            name: name.to_string(), 
            left_fork: Arc::clone(&forks[i]), // не копирует, а увеличиет счетчик ссылок на один объект 
            right_fork: Arc::clone(&forks[(i+1) % 5]), // не копирует, а увеличиет счетчик ссылок 
            thoughts: tx.clone()}).collect::<Vec<_>>(); // превращаем итерацию в коллекцию
            // в итоге получаетя вектор Vec<Arc<Mutex<Fork>>>, так как раст сам вычисляет тип
            
    
    // Дать им поесть и подумать 100 раз
    let handles = philo.into_iter() // передаёт владение into_iter
    .map(|philo| { thread::spawn(move || { // создаём новый поток spawn, move захватывает philo
    for _ in 0..100 {
        // Философ либо думает, либо ест
            philo.think();
            philo.eat();
            }})}
        ).collect::<Vec<_>>(); // превращаем итерацию в коллекцию
            
    // Вывести их мысли
    drop(tx);
    
    for dym in rx{
        println!("{}", dym);
    }
    
    for potok in handles{
        potok.join().unwrap();
    }
}