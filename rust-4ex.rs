fn magnitude(v: &[f64; 3]) -> f64 {
    let sum = v[0] * v[0] + v[1] * v[1] + v[2] * v[2];
    sum.sqrt()
}

fn normalize(v: &mut [f64; 3]) {
    let mag = magnitude(v);

    v[0] /= mag;
    v[1] /= mag;
    v[2] /= mag;
}

fn main() {
    println!("Работа с векторами");

    println!("Модуль [0,1,0]: {}", magnitude(&[0.0, 1.0, 0.0]));

    let mut v = [1.0, 2.0, 9.0];
    println!("Модуль {:?}: {}", v, magnitude(&v));

    normalize(&mut v);
    println!("После нормализации {:?}, модуль: {}", v, magnitude(&v));
}