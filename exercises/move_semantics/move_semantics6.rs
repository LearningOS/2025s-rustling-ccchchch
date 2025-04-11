fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data);  // 传递不可变引用

    string_uppercase(data);  // 传递所有权
}

// Should not take ownership
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(mut data: String) {
    data.to_uppercase();

    println!("{}", data);
}
