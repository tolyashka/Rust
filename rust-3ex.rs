fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut result = [[0; 3]; 3];

    for i in 0..3 {
        for j in 0..3 {
            result[j][i] = matrix[i][j];
        }
    }

    result
}

fn main() {
    let matrix = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];

    println!("Исходная:");
    for i in 0..3 {
        for j in 0..3 {
            print!("{} ", matrix[i][j]);
        }
        println!();
    }

    let result = transpose(matrix);

    println!("\nТранспонированная:");
    for i in 0..3 {
        for j in 0..3 {
            print!("{} ", result[i][j]);
        }
        println!();
    }
}