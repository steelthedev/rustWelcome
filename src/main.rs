fn main() {
    let mut account: BankAccount = BankAccount { 
        owner: "Alice".to_string(), 
        balance: 2000.0 
    };    

    // Immutable borrow to check the balance 
    account.check_balance();



    //Mutable borrow to withdraw money
    account.withdraw(50.0);

    account.check_balance();
}


struct BankAccount{
    owner: String,
    balance: f64,
}

impl BankAccount {
    fn withdraw(&mut self, amount: f64){
        println!("withdrawing {} from account by {}", amount, self.owner);
        self.balance -= amount;
    }

    fn check_balance(&self){
        println!("Account owned by {} has a balance of {}", self.owner, self.balance);
    }

}