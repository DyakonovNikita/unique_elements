fn unique_count(v: &mut Vec<i32>) -> usize {
    v.sort();
    v.dedup();
    v.len()
}

fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5, 1, 2, 3, 4];
    let res = unique_count(&mut numbers);
    println!("Количество уникальных элементов: {}", res)
}