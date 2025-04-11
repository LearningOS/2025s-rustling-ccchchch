fn main() {
    let vec0 = Vec::new();

    // 使用引用来传递 vec0，避免所有权转移
    let mut vec1 = fill_vec(&vec0);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: &Vec<i32>) -> Vec<i32> {
    let mut vec = vec.clone(); // 克隆 vec 来避免修改原始 vec

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
