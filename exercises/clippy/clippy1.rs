fn main() {
    let pi = std::f32::consts::PI;  // 使用标准库提供的常量
    let radius = 5.00f32;

    let area = pi * f32::powi(radius, 2);

    println!(
        "The area of a circle with radius {:.2} is {:.5}!",
        radius, area
    );
}
