fn main() {
    let number = "T-H-R-E-E"; // 字符串类型
    println!("Spell a Number : {}", number);
    
    let number = 3; // 遮蔽前一个变量，现在是整数类型
    println!("Number plus two is : {}", number + 2);
}