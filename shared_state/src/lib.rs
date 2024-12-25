use std::cell::RefCell;
use std::sync::Mutex;


//共享的资源
#[derive(Debug)]
pub struct BankAccount {
    blance: f64,
}


impl BankAccount {
    //创建一个银行账户
    pub fn new(initial_balance: f64) -> Self {
        BankAccount {blance: initial_balance}
    }

    //存款
    pub fn deposit(&mut self, amount: f64) {
        self.blance += amount;
        println!("存款成功! 当前余额: {}", self.blance);
    }

    //取款
    pub fn withdraw(&mut self, amount: f64) {
        if self.blance >= amount {
            self.blance -= amount;
            println!("取款成功! 当前余额: {}", self.blance);
        } else {
            println!("取款失败! 余额不足");
        }
    }

    //获取余额
    pub fn get_balance(&self) -> f64 {
        self.blance
    }

}

// 参与者
pub struct Person {
    name: String,
}

impl Person {
    //创建一个参与者
    pub fn new(name: String) -> Self {
        Person { name }
    }
    //尝试进行交易
    pub fn perform_transaction(&self, account: &RefCell<BankAccount>, transaction_type: &str, amount: f64) {
        println!("{} 尝试进行 {} 操作，金额: {}", self.name, transaction_type, amount);
        let mut account_borrow = account.borrow_mut();
        match transaction_type {
            "deposit" => account_borrow.deposit(amount),
            "withdraw" => account_borrow.withdraw(amount),
            _ => println!("无效的交易类型"),
        }
    }

    //尝试进行并发交易(使用Mutex)
    pub fn perform_concurrent_transaction(&self, account: &Mutex<BankAccount>, transaction_type: &str, amount: f64) {
        println!("(并发){} 尝试进行 {} 操作，金额: {}", self.name, transaction_type, amount);
        let mut account_lock = account.lock().unwrap();
        match transaction_type {
            "deposit" => account_lock.deposit(amount),
            "withdraw" => account_lock.withdraw(amount),
            _ => println!("无效的交易类型"),
        }
    }
}
