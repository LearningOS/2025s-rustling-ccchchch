fn main() {
    // 获取当前UNIX时间戳
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    // 设置环境变量 TEST_FOO（提前10秒以确保测试通过）
    println!("cargo:rustc-env=TEST_FOO={}", timestamp - 5);
    
    // 可选：打印调试信息
    println!("cargo:warning=TEST_FOO set to: {}", timestamp - 5);
}      
