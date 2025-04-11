fn main() {
    let mut res = 42;
    let option = Some(12);
    
    if let Some(x) = option {
        res += x; // 如果有值，就加上 x
    }

    println!("{}", res);
}
