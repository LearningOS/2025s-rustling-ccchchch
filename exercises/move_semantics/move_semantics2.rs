fn main() {
    let vec0 = Vec::new();

    // 克隆 vec0 并传递给 fill_vec
    let mut vec1 = fill_vec(vec0.clone());

    // 输出 vec0 内容
    println!("{} has length {}, with contents: `{:?}`", "vec0", vec0.len(), vec0);

    // 向 vec1 添加元素
    vec1.push(88);

    // 输出 vec1 内容
    println!("{} has length {}, with contents `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
