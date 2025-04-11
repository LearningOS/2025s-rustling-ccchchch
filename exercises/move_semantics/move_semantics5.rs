fn main() {
    let mut x = 100;
    let y = &mut x;
    *y += 100;

    // 这行需要移到 y 的作用域结束后，避免同时拥有多个可变引用
    let z = &mut x;
    *z += 1000;

    assert_eq!(x, 1200);
}
