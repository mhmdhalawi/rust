#[derive(Debug)]
struct Account {
    id: u32,
    balance: i32,
    holder: String,
}

impl Account {
    fn new(id: u32, holder: String) -> Self {
        Account {
            id,
            balance: 0,
            holder,
        }
    }
    fn deposit(&mut self, amount: i32) {
        self.balance += amount;
        println!("Deposited {} to account {}-{}", amount, self.id, self.holder);
    }

    fn withdraw(&mut self, amount: i32) {
        self.balance -= amount;
        println!("Withdrawn {} from account {}-{}", amount, self.id, self.holder);
    }
}

#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}

impl Bank {
    fn new() -> Self {
        Bank {
            accounts: vec![],
        }
    }

    fn add_account(&mut self, account: Account) {
        self.accounts.push(account);
    }
}

fn main() {
    let mut bank = Bank::new();

    let mut account1 = Account::new(1, "Alice".to_string());
    let mut account2 = Account::new(2, "Bob".to_string());

    account1.deposit(100);
    account2.deposit(200);

    account1.withdraw(50);

    bank.add_account(account1);
    bank.add_account(account2);

    println!("{:?}", bank);
}
