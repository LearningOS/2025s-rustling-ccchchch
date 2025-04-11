use std::collections::HashMap;

fn fruit_basket() -> HashMap<String, u32> {
    let mut basket = HashMap::new();  // 初始化一个新的 HashMap

    // 两个香蕉
    basket.insert(String::from("banana"), 2);

    // 添加更多水果
    basket.insert(String::from("apple"), 2);  // 两个苹果
    basket.insert(String::from("orange"), 1); // 一个橙子

    basket
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_least_three_types_of_fruits() {
        let basket = fruit_basket();
        assert!(basket.len() >= 3);  // 确保有至少三种水果
    }

    #[test]
    fn at_least_five_fruits() {
        let basket = fruit_basket();
        assert!(basket.values().sum::<u32>() >= 5);  // 确保水果的总数至少为五个
    }
}
