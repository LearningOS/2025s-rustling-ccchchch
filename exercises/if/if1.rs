pub fn bigger(a: i32, b: i32) -> i32 {
    // 使用 if 语句返回较大的数
    if a > b {
        a
    } else {
        b
    }
}

// 测试代码
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ten_is_bigger_than_eight() {
        assert_eq!(10, bigger(10, 8));
    }

    #[test]
    fn fortytwo_is_bigger_than_thirtytwo() {
        assert_eq!(42, bigger(32, 42));
    }
}
