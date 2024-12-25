// 引入 lib.rs 中定义的函数
use ownership::reverse_string;

fn main() {
    // 创建一个初始字符串
    let original = String::from("Rust Ownership");
    // 调用 reverse_string 函数，传入 original 的所有权
    let reversed = reverse_string(original);
    // 打印反转后的字符串
    println!("反转后的字符串: {}", reversed);
    // 尝试访问 original 字符串，会报错
    // println!("original: {}", original);
}
