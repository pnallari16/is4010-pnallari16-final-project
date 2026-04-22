use serde::{Serialize, Deserialize};
use chrono::{NaiveDate, Utc};

// 1. Define the type of transaction
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum TransactionType {
    Income,
    Expense,
}

// 2. Define the Transaction structure
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub id: u32,
    pub date: NaiveDate,
    pub description: String,
    pub amount: f64,
    pub kind: TransactionType,
}

// 3. Define the Tracker to hold transactions
pub struct FinanceTracker {
    pub transactions: Vec<Transaction>,
}

impl FinanceTracker {
    pub fn new() -> Self {
        Self { transactions: Vec::new() }
    }

    pub fn add_transaction(&mut self, description: String, amount: f64, kind: TransactionType) {
        let id = (self.transactions.len() as u32) + 1;
        let date = Utc::now().date_naive();
        self.transactions.push(Transaction { id, date, description, amount, kind });
    }

    pub fn get_balance(&self) -> f64 {
        self.transactions.iter().fold(0.0, |acc, t| {
            match t.kind {
                TransactionType::Income => acc + t.amount,
                TransactionType::Expense => acc - t.amount,
            }
        })
    }
}

// 4. Main function to run the program
fn main() {
    let mut my_tracker = FinanceTracker::new();
    
    println!("--- Welcome to your Rust Finance Tracker ---");
    
    my_tracker.add_transaction("Paycheck".to_string(), 2500.0, TransactionType::Income);
    my_tracker.add_transaction("Rent".to_string(), 1200.0, TransactionType::Expense);
    
    println!("Current Balance: ${}", my_tracker.get_balance());
}

// 5. Testing Suite (Requirement #3)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_tracker_is_empty() {
        let tracker = FinanceTracker::new();
        assert_eq!(tracker.transactions.len(), 0);
    }

    #[test]
    fn test_add_income() {
        let mut tracker = FinanceTracker::new();
        tracker.add_transaction("Bonus".into(), 500.0, TransactionType::Income);
        assert_eq!(tracker.get_balance(), 500.0);
    }

    #[test]
    fn test_add_expense() {
        let mut tracker = FinanceTracker::new();
        tracker.add_transaction("Coffee".into(), 5.0, TransactionType::Expense);
        assert_eq!(tracker.get_balance(), -5.0);
    }

    #[test]
    fn test_multiple_transactions() {
        let mut tracker = FinanceTracker::new();
        tracker.add_transaction("Salary".into(), 1000.0, TransactionType::Income);
        tracker.add_transaction("Groceries".into(), 200.0, TransactionType::Expense);
        assert_eq!(tracker.get_balance(), 800.0);
    }

    #[test]
    fn test_transaction_ids() {
        let mut tracker = FinanceTracker::new();
        tracker.add_transaction("A".into(), 1.0, TransactionType::Income);
        tracker.add_transaction("B".into(), 1.0, TransactionType::Income);
        assert_eq!(tracker.transactions[0].id, 1);
        assert_eq!(tracker.transactions[1].id, 2);
    }
}
