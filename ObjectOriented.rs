fn main(){
    let mut account: BankAccount = BankAccount{
        owner: "William".to_string(),
        balance: 565.58,
    };
    // Immutable borrow to check the balance
    account.withdraw(65.58);
    account.check_balance();
}
struct BankAccount{
    owner: String,
    balance: f64,
}
impl BankAccount{
    fn withdraw(&mut self, amount: f64){
        println!("Withdrawing {} from account owned by {}",amount,self.owner);
        self.balance -= amount;
    }
    fn check_balance(&self){
        println!("Account owned by {} has a balance of {}",self.owner, self.balance);
    }
}

