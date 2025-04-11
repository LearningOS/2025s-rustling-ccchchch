fn maybe_icecream(time_of_day: u16) -> Option<u16> {
    if time_of_day > 23 {
        return None;
    }
    if time_of_day >= 22 {
        Some(0)
    } else {
        Some(5)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_icecream() {
        assert_eq!(maybe_icecream(9), Some(5));
        assert_eq!(maybe_icecream(10), Some(5));
        assert_eq!(maybe_icecream(23), Some(0));
        assert_eq!(maybe_icecream(22), Some(0));
        assert_eq!(maybe_icecream(25), None);
    }

    #[test]
    fn raw_value() {
        // 获取 Option 中的值
        let icecreams = maybe_icecream(12).unwrap();  // 使用 unwrap 获取 Option 中的值
        assert_eq!(icecreams, 5);
    }
}
