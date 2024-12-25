use book_management::{Book, persistence};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // 尝试从文件加载图书馆数据
    let mut library = persistence::load_library_from_file()?;

    // 创建一些书籍
    let book1 = Book::new("The Lord of the Rings".to_string(), "J.R.R. Tolkien".to_string(), "978-0618260274".to_string());
    let book2 = Book::new("Pride and Prejudice".to_string(), "Jane Austen".to_string(), "978-0141439518".to_string());
    let book3 = Book::new("Rust in Action".to_string(), "Tim McNamara".to_string(), "978-1617294639".to_string());

    // 将书籍添加到图书馆
    library.add_book(book1)?;
    library.add_book(book2)?;
    library.add_book(book3)?;

    println!("\n--- 图书馆信息 ---");
    library.list_all_books();

    // 查找书籍
    println!("\n--- 通过 ISBN 查找 ---");
    match library.find_book_by_isbn("978-0141439518") {
        Ok(book) => println!("找到书籍：{}，作者：{}", book.title, book.author),
        Err(_) => println!("未找到该 ISBN 的书籍。"),
    }

    // 借阅和归还书籍
    println!("\n--- 借阅和归还 ---");
    library.borrow_book("978-0618260274", "Alice")?;
    library.list_all_books();
    library.return_book("978-0618260274")?;
    library.list_all_books();

    println!("\n--- Alice 借阅的书籍 ---");
    for book in library.find_borrowed_by("Alice") {
        println!("- {}", book.title);
    }

    // 保存图书馆数据到文件
    persistence::save_library_to_file(&library)?;

    Ok(())
}