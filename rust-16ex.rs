/// Вычислите разность между значениями values на расстоянии offset друг от друга,
/// переходя по модулю в начало коллекции.
///
/// Элемент n результата это разность values[(n+offset)%len] - values[n].
fn offset_differences(offset: usize, values: Vec<i32>) -> Vec<i32> {
    let len = values.len();
    
    if len == 0 {
        return Vec::new();
    }
    // Для каждого индекса i находим элемент, 
    // который находится через offset позиций вперед, 
    // и вычитает из него текущий элемен
    
    (0..len)
        .map(|i| { // применяем вычисления на каждой итерации. i - индекс элемента вектора
        // 
            let current = values[i];
            let next_index = (i + offset) % len;
            let next = values[next_index];
            next - current
        })
        .collect() // выполняет итерацию и собирает в Vec<i32>
}

#[test]
fn test_offset_one() {
// [1, 3, 5, 7], offset: 1. 
// i = 0
// value = 1 
// (i+1)%4 = 1
// values[(i+1)%4] = 3 
// 3 - 1 = 2 

// i = 1
// value = 3
// (i+1)%4 = 2
// values[(i+1)%4] = 5	
// 5 - 3 = 2


// i = 3 (последний индекс)
// current = values[3] = 7
// next_index = (3 + 1) % 4 = 4 % 4 = 0
// next = values[0] = 1
// результат = 1 - 7 = -6
    assert_eq!(offset_differences(1, vec![1, 3, 5, 7]), vec![2, 2, 2, -6]);
    assert_eq!(offset_differences(1, vec![1, 3, 5]), vec![2, 2, -4]);
    assert_eq!(offset_differences(1, vec![1, 3]), vec![2, -2]);
}

#[test]
fn test_larger_offsets() {
    assert_eq!(offset_differences(2, vec![1, 3, 5, 7]), vec![4, 4, -4, -4]);
    assert_eq!(offset_differences(3, vec![1, 3, 5, 7]), vec![6, -2, -2, -2]);
    assert_eq!(offset_differences(4, vec![1, 3, 5, 7]), vec![0, 0, 0, 0]);
    assert_eq!(offset_differences(5, vec![1, 3, 5, 7]), vec![2, 2, 2, -6]);
}

#[test]
fn test_degenerate_cases() {
    assert_eq!(offset_differences(1, vec![0]), vec![0]);
    assert_eq!(offset_differences(1, vec![1]), vec![0]);
    let empty: Vec<i32> = vec![];
    assert_eq!(offset_differences(1, empty), vec![]);
}
