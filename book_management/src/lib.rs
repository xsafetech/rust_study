use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use std::io;
use std::fmt;
use std::error::Error;

// 定义书籍结构体，添加借阅状态
#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct Book {
    pub title: String, // 书籍的标题
    pub author: String, // 书籍的作者
    pub isbn: String, // 书籍的唯一标识符
    pub borrowed_by: Option<String>, // 借阅者
}

impl Book {
    pub fn new(title: String, author: String, isbn: String) -> Self {
        Book { title, author, isbn, borrowed_by: None }
    }
}

// 定义图书馆结构体
#[derive(Debug, Serialize, Deserialize)]
pub struct Library {
    books: HashMap<String, Book>,
}

// 定义一个 Result 类型，用于表示图书馆操作的结果
pub type LibraryResult<T> = Result<T, LibraryError>;

// 定义图书馆操作可能产生的错误类型
#[derive(Debug, PartialEq)]
pub enum LibraryError {
    BookAlreadyExists,
    BookNotFound,
    BookAlreadyBorrowed,
    BookNotBorrowed,
    IoError(String),
    SerdeError(String), // 添加处理 serde_json 错误的变体
}

// 将 io::Error 转换为 LibraryError
impl From<io::Error> for LibraryError {
    fn from(err: io::Error) -> Self {
        LibraryError::IoError(err.to_string())
    }
}
// 实现 From<serde_json::Error> for LibraryError
impl From<serde_json::Error> for LibraryError {
    fn from(error: serde_json::Error) -> Self {
        LibraryError::SerdeError(error.to_string())
    }
}
// 实现 Display trait
impl fmt::Display for LibraryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LibraryError::BookAlreadyExists => write!(f, "书籍已存在"),
            LibraryError::BookNotFound => write!(f, "未找到书籍"),
            LibraryError::BookAlreadyBorrowed => write!(f, "书籍已被借出"),
            LibraryError::BookNotBorrowed => write!(f, "书籍未被借出"),
            LibraryError::IoError(e) => write!(f, "IO 错误: {}", e),
            LibraryError::SerdeError(e) => write!(f, "Serde 错误: {}", e),
        }
    }
}
// 实现 Error trait
impl Error for LibraryError {}
   
// 实现 Library 结构体的方法
impl Library {
    pub fn new() -> Self {
        Library { books: HashMap::new() }
    }

    // 添加书籍，返回 LibraryResult
    pub fn add_book(&mut self, book: Book) -> LibraryResult<()> {
        // 检查书籍是否已存在
        if self.books.contains_key(&book.isbn) {
            Err(LibraryError::BookAlreadyExists)
        } else {
            self.books.insert(book.isbn.clone(), book.clone());
            println!("书籍 '{}' 已添加到图书馆。", book.title);
            Ok(())
        }
    }
    // 通过 ISBN 查找书籍，返回 LibraryResult
    pub fn find_book_by_isbn(&self, isbn: &str) -> LibraryResult<&Book> {
        self.books.get(isbn).ok_or(LibraryError::BookNotFound)
    }

    // 通过作者查找书籍
    pub fn find_books_by_author(&self, author: &str) -> Vec<&Book> {
        self.books.values().filter(|book| book.author == author).collect() // 返回所有作者匹配的书籍
    }

    // 列出所有书籍
    pub fn list_all_books(&self) {
        if self.books.is_empty() {
            println!("图书馆目前没有书籍。");
        } else {
            println!("图书馆现有书籍: ");
            for book in self.books.values() {
                let status = if book.borrowed_by.is_some() { "（已借出）" } else { "" };
                println!("- {}，作者：{}，ISBN：{}{}", book.title, book.author, book.isbn, status);
            }
        }
    }

    // 借阅书籍
    pub fn borrow_book(&mut self, isbn: &str, borrower: &str) -> LibraryResult<()> {
        // 获取书籍的可变引用
        match self.books.get_mut(isbn) {
            // 如果书籍存在，检查是否已被借阅
            Some(book) => {
                // 如果书籍已被借阅，返回错误
                if book.borrowed_by.is_some() {
                    Err(LibraryError::BookAlreadyBorrowed)
                } else {
                    // 如果书籍未被借阅，设置借阅者
                    book.borrowed_by = Some(borrower.to_string());
                    println!("书籍 '{}' 已被 {} 借阅。", book.title, borrower);
                    Ok(())
                }
            }
            // 如果书籍不存在，返回错误
            None => Err(LibraryError::BookNotFound),
        }
    }

    // 归还书籍
    pub fn return_book(&mut self, isbn: &str) -> LibraryResult<()> {
        // 获取书籍的可变引用
        match self.books.get_mut(isbn) {
            // 如果书籍存在，检查是否已被归还
            Some(book) => {
                // 如果书籍已被归还，设置 借阅者为 None
                if book.borrowed_by.is_some() {
                    println!("书籍 '{}' 已归还。", book.title);
                    book.borrowed_by = None;
                    Ok(())
                } else {
                    Err(LibraryError::BookNotBorrowed)
                }
            }
            None => Err(LibraryError::BookNotFound),
        }
    }
    // 查找被特定用户借阅的书籍
    pub fn find_borrowed_by(&self, borrower: &str) -> Vec<&Book> {
        self.books
            .values()
            .filter(|book| book.borrowed_by == Some(borrower.to_string()))
            .collect()
    }    
}

// 定义一个 Trait 用于抽象不同的图书存储方式
pub trait BookStore{
    // 添加书籍
    fn add_book(&mut self, book: Book) -> LibraryResult<()>;
    // 通过 ISBN 查找书籍
    fn get_book(&self, isbn: &str) -> LibraryResult<&Book>;
    // 通过作者查找书籍
    fn get_books_by_author(&self, author: &str) -> Vec<&Book>;
    // 列出所有书籍
    fn all_books(&self) -> Vec<&Book>;
}

// 实现 BookStore Trait 的默认方法
impl BookStore for Library {
    fn add_book(&mut self, book: Book) -> LibraryResult<()> {
        Library::add_book(self, book)
    }

    fn get_book(&self, isbn: &str) -> LibraryResult<&Book> {
        Library::find_book_by_isbn(self, isbn)
    }

    fn get_books_by_author(&self, author: &str) -> Vec<&Book> {
        Library::find_books_by_author(self, author)
    }

    fn all_books(&self) -> Vec<&Book> {
        self.books.values().collect()
    }
}

// 定义一个模块用于图书数据的持久化
pub mod persistence {
    use std::fs::File;
    use std::io::{BufReader, BufWriter};
    use super::*;

    const LIBRARY_DATA_FILE: &str = "library_data.json";

    pub fn save_library_to_file(library: &Library) -> LibraryResult<()> {
        let file = File::create(LIBRARY_DATA_FILE)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, library)
            .map_err(|e| LibraryError::SerdeError(e.to_string()))?; // 显式转换
        Ok(())
    }

    pub fn load_library_from_file() -> LibraryResult<Library> {
        match File::open(LIBRARY_DATA_FILE) {
            Ok(file) => {
                let reader = BufReader::new(file);
                serde_json::from_reader(reader)
                    .map_err(|e| LibraryError::SerdeError(e.to_string())) // 显式转换
            }
            Err(_) => {
                println!("未找到图书馆数据文件，创建一个新的图书馆。");
                Ok(Library::new())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_and_find_book() {
        let mut library = Library::new();
        let book1 = Book::new("Rust Programming".to_string(), "John Doe".to_string(), "123-456".to_string());
        library.add_book(book1.clone()).unwrap();
        assert_eq!(library.find_book_by_isbn("123-456").unwrap().title, "Rust Programming");
    }

    #[test]
    fn test_add_duplicate_book() {
        let mut library = Library::new();
        let book1 = Book::new("Rust Programming".to_string(), "John Doe".to_string(), "123-456".to_string());
        library.add_book(book1.clone()).unwrap();
        let result = library.add_book(book1);
        assert_eq!(result, Err(LibraryError::BookAlreadyExists));
    }

    #[test]
    fn test_borrow_and_return_book() {
        let mut library = Library::new();
        let book1 = Book::new("Rust Programming".to_string(), "John Doe".to_string(), "123-456".to_string());
        library.add_book(book1.clone()).unwrap();

        library.borrow_book("123-456", "Alice").unwrap();
        assert_eq!(library.find_book_by_isbn("123-456").unwrap().borrowed_by, Some("Alice".to_string()));

        library.return_book("123-456").unwrap();
        assert_eq!(library.find_book_by_isbn("123-456").unwrap().borrowed_by, None);
    }

    #[test]
    fn test_borrow_nonexistent_book() {
        let mut library = Library::new();
        let result = library.borrow_book("nonexistent", "Alice");
        assert_eq!(result, Err(LibraryError::BookNotFound));
    }

    #[test]
    fn test_borrow_already_borrowed_book() {
        let mut library = Library::new();
        let book1 = Book::new("Rust Programming".to_string(), "John Doe".to_string(), "123-456".to_string());
        library.add_book(book1.clone()).unwrap();
        library.borrow_book("123-456", "Alice").unwrap();
        let result = library.borrow_book("123-456", "Bob");
        assert_eq!(result, Err(LibraryError::BookAlreadyBorrowed));
    }
}