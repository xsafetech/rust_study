use std::fmt::Debug;

// 定义一个 Trait，表示可以进行安全交换的类型
pub trait Swappable: Debug {
    fn can_swap(&self, other: &Self) -> bool;
}

impl Swappable for i32 {
    fn can_swap(&self, other: &Self) -> bool {
        // 只有当两个数字的绝对值之差小于 10 时才允许交换
        self.abs_diff(*other) < 10
    }
}

// 复杂的交换函数，使用泛型和 Trait 约束
pub fn complex_swap<T: Swappable>(a: &mut T, b: &mut T) -> Result<(), &'static str> {
    println!("尝试交换: a = {:?}, b = {:?}", a, b);
    if a.can_swap(b) && b.can_swap(a) {
        // 使用 std::mem::swap 安全地交换值
        std::mem::swap(a, b);
        println!("成功交换: a = {:?}, b = {:?}", a, b);
        Ok(())
    } else {
        Err("无法交换：不满足交换条件")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_complex_swap_success() {
        let mut x = 5;
        let mut y = 8;
        assert_eq!(complex_swap(&mut x, &mut y), Ok(()));
        assert_eq!(x, 8);
        assert_eq!(y, 5);
    }

    #[test]
    fn test_complex_swap_failure() {
        let mut x = 5;
        let mut y = 100;
        assert_eq!(complex_swap(&mut x, &mut y), Err("无法交换：不满足交换条件"));
        assert_eq!(x, 5);
        assert_eq!(y, 100);
    }
}