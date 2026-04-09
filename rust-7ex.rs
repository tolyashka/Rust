// Логгер — интерфейс для вывода сообщений с уровнем важности
pub trait Logger {
    fn log(&self, verbosity: u8, message: &str);
}

// Простой логгер (печатает в консоль)
struct StdErrLogger;

impl Logger for StdErrLogger {
    fn log(&self, verbosity: u8, message: &str) {
        println!("verbosity={verbosity} : {message}");
    }
}

// Фильтр: пропускает только сообщения с заданным уровнем
struct VerbosityFilter {
    max_verbosity: u8,
    inner: StdErrLogger,
}

impl Logger for VerbosityFilter {
    fn log(&self, verbosity: u8, message: &str) {
        if verbosity == self.max_verbosity {
            self.inner.log(verbosity, message);
        }
    }
}

fn main() {
    let logger = VerbosityFilter {
        max_verbosity: 3,
        inner: StdErrLogger,
    };

    logger.log(5, "Не выведется");
    logger.log(3, "Выведется");
}