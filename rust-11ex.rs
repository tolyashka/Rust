// Используем тот же Logger
pub trait Logger {
    fn log(&self, verbosity: u8, message: &str);
}

// Простой логгер
struct StdErrLogger;

impl Logger for StdErrLogger {
    fn log(&self, verbosity: u8, message: &str) {
        println!("verbosity={verbosity} : {message}");
    }
}

// Обобщённый фильтр с замыканием
struct FilterLogger<InnerLogger, Predicate>
where
    InnerLogger: Logger,
    Predicate: Fn(u8, &str) -> bool,
{
    inner_logger: InnerLogger,
    filter_predicate: Predicate,
}

impl<InnerLogger, Predicate> FilterLogger<InnerLogger, Predicate>
where
    InnerLogger: Logger,
    Predicate: Fn(u8, &str) -> bool,
{
    fn new(inner_logger: InnerLogger, filter_predicate: Predicate) -> Self {
        Self {
            inner_logger,
            filter_predicate,
        }
    }
}

impl<InnerLogger, Predicate> Logger for FilterLogger<InnerLogger, Predicate>
where
    InnerLogger: Logger,
    Predicate: Fn(u8, &str) -> bool,
{
    fn log(&self, level: u8, message: &str) {
        if (self.filter_predicate)(level, message) {
            self.inner_logger.log(level, message);
        }
    }
}

fn main() {
    // Пропускаем только сообщения с уровнем <= 3
    let logger = FilterLogger::new(StdErrLogger, |lvl, _| lvl <= 3);

    logger.log(5, "Не выведется");
    logger.log(2, "Выведется");
}