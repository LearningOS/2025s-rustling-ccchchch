mod macros {
    #[macro_export]
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

fn main() {
    crate::my_macro!(); // 用 crate:: 访问导出的宏
}
