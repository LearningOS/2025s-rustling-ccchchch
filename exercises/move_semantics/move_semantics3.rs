fn main() {
    let mut vec0 = Vec::new();  // 将 vec0 声明为可变

    let mut vec1 = fill_vec(&mut vec0);  // 传递 vec0 的可变引用

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: &mut Vec<i32>) -> Vec<i32> {
    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec.clone()  // 返回一个克隆的 Vec，这样就可以在 main 中使用了
}
