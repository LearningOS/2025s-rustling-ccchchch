fn main() {
    let answer = square(3);  // 计算 3 的平方
    println!("The square of 3 is {}", answer);  // 输出结果
}

fn square(num: i32) -> i32 {  // 定义计算平方的函数
    num * num  // 返回 num 的平方
}
