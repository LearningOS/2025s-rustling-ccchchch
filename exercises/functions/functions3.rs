fn main() {
    call_me(3);  // 调用 call_me 函数，传入参数 3
}

fn call_me(num: u32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);  // 打印消息，i + 1 用来从 1 开始
    }
}
