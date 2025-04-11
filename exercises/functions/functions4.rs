fn main() {
    let original_price = 51;  // 设置原价
    println!("Your sale price is {}", sale_price(original_price));  // 输出打折后的价格
}

fn sale_price(price: i32) -> i32 {  // 计算打折后价格的函数
    if is_even(price) {  // 如果是偶数
        price - 10  // 打 10 Rustbucks 的折扣
    } else {  // 如果是奇数
        price - 3  // 打 3 Rustbucks 的折扣
    }
}

fn is_even(num: i32) -> bool {  // 判断是否是偶数
    num % 2 == 0  // 偶数条件：num % 2 等于 0
}
