pub fn reverse_string(s: String) -> String {
    println!("reverse_string 接收到了字符串的所有权: {}", s);

    let reversed: String = s.chars().rev().collect();

    println!("reverse_string 创建了新的反转字符串: {}", reversed);
    reversed // 返回反转后的字符串，所有权转移
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_string() {
        let original = String::from("hello");
        let reversed = reverse_string(original);
        assert_eq!(reversed, "olleh");
        // 注意：此时 original 的所有权已经转移到 reverse_string 函数中，无法再使用
    }
}
