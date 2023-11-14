use std::io;
use std::str::FromStr;

fn unique_count(v: &mut Vec<i32>) -> usize {
    v.sort();
    v.dedup();
    v.len()
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Не удалось прочитать строку");

    let mut numbers: Vec<i32> = input
        .trim()
        .split(", ")
        .map(|s| i32::from_str(s).expect("Не удалось преобразовать строку в число"))
        .collect();
    // let mut numbers = vec![1, 2, 3, 4, 5, 1, 2, 3, 4];
    let res = unique_count(&mut numbers);
    println!("Количество уникальных элементов: {}", res)
}