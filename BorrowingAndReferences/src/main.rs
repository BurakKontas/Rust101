#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_mut)]

fn main() {
    // references();
    bank();
}

fn bank() {
    let mut account: BankAccount = BankAccount {
        owner: "Alice".to_string(),
        balance: 100.55
    };

    account.check_balance();

    account.deposit(50.12);
    account.check_balance();

    account.withdraw(25.36);
    account.check_balance();
}

fn references() {
    let x: i32 = 5;
    let r: &i32 = &x;

    // *r += 1; // error: cannot assign to `*r`, which is behind a `&` reference `r` is a `&` reference, so the data it refers to cannot be written

    let mut y: i32 = 5;
    let m: &mut i32 = &mut y;

    *m += 1;
    *m -= 3;

    println!("y = {}", y);

    // you can have 1 mutable reference or multiple immutable references, but not both
}

struct BankAccount {
    owner: String,
    balance: f64
}

impl BankAccount {
    fn withdraw(&mut self, amount: f64) {
        println!("Withdrawing ${:.2 } from account owned by {}", amount, self.owner);
        self.balance -= amount;
    }

    fn deposit(&mut self, amount: f64) {
        println!("Depositing ${:.2} into account owned by {}", amount, self.owner);
        self.balance += amount;
    }

    fn check_balance(&self) {
        println!("Current balance: ${:.2}", self.balance);
    }
}