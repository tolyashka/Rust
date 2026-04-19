// Алгоритм Луна 18 задание 
// Алгоритм используется для проверки номеров кредитных карт
// Пробелы игнорируются, номер должен иметь минимум 2 цифры
// Каждая вторая цифра считая справа налево увеличивается вдвое, если результат больше 9, то десятки и единицы результата складываются
// Все числа, полученные вышеописанным способом, и остальные числа
// номера складываются
// Если полученная сумма кратна 10, номер карты верный
// Допишите юнит тесты к имеющейся программе для того, чтобы 

// Берём число справа налево
// каждую вторую цифру удваиваем
// если результат > 9 то  вычитаем 9
// суммируем всё
// если сумма делится на 10 - номер валиден

pub fn luhn(cc_number: &str) -> bool {
    let mut sum = 0;
    let mut double = false;
    let mut digits_found = 0;

    for c in cc_number.chars().rev() {
        if c == ' ' {
            continue;
        }

        let digit = match c.to_digit(10) {
            Some(d) => d,
            None => return false,
        };

        digits_found += 1;

        if double {
            let double_digit = digit * 2;
            sum += if double_digit > 9 {
                double_digit - 9
            } else {
                double_digit
            };
        } else {
            sum += digit;
        }

        double = !double;
    }

    if digits_found < 2 {
        return false;
    }

    sum % 10 == 0
}

#[cfg(test)]
mod test {
    use super::*;

    // Исходные тесты
    #[test]
    fn test_valid_cc_number() {
        // assert!(luhn("ddd"));
        assert!(luhn("123"));
        // 3 
        // 2 + 2 
        // 1 
        // 3 + 4 + 1 = 8 % 10 != 0 
        assert!(luhn("4263 9826 4026 9299"));
        assert!(luhn("4539 3195 0343 6467"));
        assert!(luhn("7992 7398 713")); // 
    }

    #[test]
    fn test_invalid_cc_number() {
        assert!(!luhn("4223 9826 4026 9299"));
        assert!(!luhn("4539 3195 0343 6476"));
        assert!(!luhn("8273 1232 7352 0514")); // 
    }

    // Дополнительные теститки 
    #[test]
    fn test_empty_string() {
        assert!(!luhn(""));
    }

    #[test]
    fn test_single_digit_zero() {
        assert!(!luhn("0"));
    }

    #[test]
    fn test_only_spaces() {
        assert!(!luhn("   "));
    }

    #[test]
    fn test_non_space_separators() {
        assert!(!luhn("4539-3195-0343-6467"));
    }

    #[test]
    fn test_letters_mixed() {
        assert!(!luhn("abc4539319503436467"));
    }
}