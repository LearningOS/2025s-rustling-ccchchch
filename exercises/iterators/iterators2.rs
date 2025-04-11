// Step 1.
// 完成 capitalize_first 函数
pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    match c.next() {
        None => String::new(),
        Some(first) => {
            let capitalized = first.to_uppercase().collect::<String>();
            capitalized + c.as_str()
        },
    }
}

// Step 2.
// 对字符串切片应用 capitalize_first 函数
// 返回字符串向量
pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    words.iter()
        .map(|word| capitalize_first(word))
        .collect()
}

// Step 3.
// 再次对字符串切片应用 capitalize_first 函数
// 返回单个字符串
pub fn capitalize_words_string(words: &[&str]) -> String {
    words.iter()
        .map(|word| capitalize_first(word))
        .collect::<Vec<String>>()
        .join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}