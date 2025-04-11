#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None;

    // 使用 is_none() 代替冗余的模式匹配
    if my_option.is_none() {
        println!("Option is None");
    }

    // 修正数组声明中的问题，确保元素之间有逗号
    let my_arr = &[
        -1, -2, -3, // 需要加逗号
        -4, -5, -6,
    ];
    println!("My array! Here it is: {:?}", my_arr);

    // 清空 Vec 使用 clear，而不是 resize
    let mut my_empty_vec = vec![1, 2, 3, 4, 5];
    my_empty_vec.clear();
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    // 交换两个变量的值，使用 std::mem::swap
    let mut value_a = 45;
    let mut value_b = 66;
    // 使用 std::mem::swap 来交换值
    std::mem::swap(&mut value_a, &mut value_b);
    println!("value a: {}; value b: {}", value_a, value_b);
}
