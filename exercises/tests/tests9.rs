// exercises/tests/tests9.rs

// 修复模块名称为蛇形命名
mod foo {
    // 导出函数供测试使用
    pub fn my_demo_function(a: u32) -> u32 {
        a
    }

    // 为函数创建别名
    pub use my_demo_function as my_demo_function_alias;
}

#[cfg(test)]
mod tests {
    // 引入需要测试的函数
    use super::foo::{my_demo_function, my_demo_function_alias};

    #[test]
    fn test_success() {
        // 测试原始函数
        assert_eq!(my_demo_function(42), 42);
        // 测试函数别名
        assert_eq!(my_demo_function_alias(42), 42);
    }
}