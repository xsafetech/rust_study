use shared_state::{BankAccount, Person};
use std::cell::RefCell;
use std::rc::Rc;
use std::thread;
use std::sync::{Mutex, Arc};

fn main() {
    println!("---单线程环境下的贡献状态(使用 RefCell) ---");
    let account = Rc::new(RefCell::new(BankAccount::new(1000.0)));

    let person1 = Person::new("Alice".to_string());
    let person2 = Person::new("Bob".to_string());

    //Alice 进行存款
    person1.perform_transaction(&account, "deposit", 500.0);
    //Bob 进行取款
    person2.perform_transaction(&account, "withdraw", 200.0);

    println!("最终账户余额: {}", account.borrow().get_balance());

    println!("---多线程环境下的贡献状态(使用 Mutex) ---");

    //创建共享账户
    let shared_account = Arc::new(Mutex::new(BankAccount::new(5000.0)));
    //克隆共享账户
    let shared_account_clone1 = Arc::clone(&shared_account);
    //Alice 进行存款
    let thread1 = thread::spawn(move || {
        let person = Person::new("Thread-Alice".to_string());
        person.perform_concurrent_transaction(&shared_account_clone1, "deposit", 1000.0);
    });

    
    //克隆共享账户
    let shared_account_clone2 = Arc::clone(&shared_account);
    //Bob 进行取款
    let thread2 = thread::spawn(move || {
        let person = Person::new("Thread-Bob".to_string());
        person.perform_concurrent_transaction(&shared_account_clone2, "withdraw", 2000.0);
    });

    //等待线程完成
    thread1.join().unwrap();
    thread2.join().unwrap();

    println!("最终账户余额 (多线程): {}", shared_account.lock().unwrap().get_balance());


    println!("\n--- 演示 RefCell 的运行时借用检查 (可能导致 Panic) ---");
    //创建一个共享账户
    let risky_account = Rc::new(RefCell::new(BankAccount::new(100.0)));
    //克隆共享账户
    let risky_account_clone = Rc::clone(&risky_account);
    // 尝试同时获取可变借用，会导致运行时 Panic
    let _borrow1 = risky_account.borrow_mut();
    let _borrow2 = risky_account_clone.borrow_mut(); // 取消注释这行会触发 Panic
    println!("这段代码可能不会执行到，如果上面的 borrow_mut() 触发了 Panic");
}