mod sausage_factory {
    // 不让外部访问这个函数
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

    // 允许外部访问这个函数
    pub fn make_sausage() {
        get_secret_recipe();
        println!("sausage!");
    }
}

fn main() {
    sausage_factory::make_sausage();
}
