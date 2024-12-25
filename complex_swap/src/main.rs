use complex_swap::complex_swap;

fn main() {
    let mut num1 = 25;
    let mut num2 = 20;

    println!("交换前: num1 = {}, num2 = {}", num1, num2);
    match complex_swap(&mut num1, &mut num2) {
        Ok(_) => println!("交换成功! num1 = {}, num2 = {}", num1, num2),
        Err(e) => println!("交换失败: {}", e),
    }

    let mut val1 = 5;
    let mut val2 = 100;

    println!("交换前: val1 = {}, val2 = {}", val1, val2);
    match complex_swap(&mut val1, &mut val2) {
        Ok(_) => println!("交换成功! val1 = {}, val2 = {}", val1, val2),
        Err(e) => println!("交换失败: {}", e),
    }
}
