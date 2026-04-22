use serde::{Serialize, Deserialize};
use chrono::{NaiveDate, Utc};
use std::fs::File;
use std::io::{self, Write, Read};
use std::path::Path;

// --- Data Structures ---

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum TransactionType {
    Income,
    Expense,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub id: u32,
    pub date: NaiveDate,
    pub description: String,
    pub amount: f64,
    pub kind: TransactionType,
}

pub struct FinanceTracker {
    pub transactions: Vec<Transaction>,
}

// --- Implementation Logic ---

impl FinanceTracker {
    pub fn new() -> Self {
        Self { transactions: Vec::new() }
    }

    pub fn add_transaction(&mut self, description: String, amount: f64, kind: TransactionType) {
        let id = (self.transactions.len() as u32) + 1;
        let date = Utc::now().date_naive();
        self.transactions.push(Transaction {
            id,
            date,
            description,
            amount,
            kind,
        });
    }

    pub fn get_balance(&self) -> f64 {
        self.transactions.iter().fold(0.0, |acc, t| {
            match t.kind {
                TransactionType::Income => acc + t.amount,
                TransactionType::Expense => acc - t.amount,
            }
        })
    }

    pub fn save_to_file(&self, filename: &str) -> io::Result<()> {
        let json = serde_json::to_string_pretty(&self.transactions)?;
        let mut file = File::create(filename)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }

    pub fn load_from_file(&mut self, filename: &str) -> io::Result<()> {
        if Path::new(filename).exists() {
            let mut file = File::open(filename)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            self.transactions = serde_json::from_str(&contents)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        }
        Ok(())
    }
}

// --- Main Interactive Loop ---

fn main() {
    let mut my_tracker = FinanceTracker::new();
    let file_path = "transactions.json";

    // Load existing data on startup
    if let Err(e) = my_tracker.load_from_file(file_path) {
        println!("Note: Could not load existing data ({}). Starting fresh.", e);
    }

    println!("--- Welcome to the Rust Finance Tracker ---");

    loop {
        println!("\n--- Menu ---");
        println!("1. Add Income");
        println!("2. Add Expense");
        println!("3. View Current Balance");
        println!("4. List All Transactions");
        println!("5. Save and Exit");
        print!("Choose an option: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");

        match choice.trim() {
            "1" => {
                let (desc, amount) = get_transaction_input("Income");
                my_tracker.add_transaction(desc, amount, TransactionType::Income);
                my_tracker.save_to_file(file_path).expect("Failed to auto-save");
                println!("Income recorded.");
            }
            "2" => {
                let (desc, amount) = get_transaction_input("Expense");
                my_tracker.add_transaction(desc, amount, TransactionType::Expense);
                my_tracker.save_to_file(file_path).expect("Failed to auto-save");
                println!("Expense recorded.");
            }
            "3" => {
                println!("\n>>> Total Balance: ${:.2}", my_tracker.get_balance());
            }
            "4" => {
                println!("\n--- History ---");
                if my_tracker.transactions.is_empty() {
                    println!("No transactions found.");
                } else {
                    for t in &my_tracker.transactions {
                        println!("[ID: {}] {} | {:?}: {} - ${:.2}", t.id, t.date, t.kind, t.description, t.amount);
                    }
                }
            }
            "5" => {
                my_tracker.save_to_file(file_path).expect("Final save failed");
                println!("Data saved. Goodbye!");
                break;
            }
            _ => println!("Invalid selection. Please try again."),
        }
    }
}

fn get_transaction_input(label: &str) -> (String, f64) {
    let mut desc = String::new();
    let mut amount_str = String::new();

    print!("Enter {} description: ", label);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut desc).expect("Failed to read description");

    loop {
        print!("Enter amount: ");
        io::stdout().flush().unwrap();
        amount_str.clear();
        io::stdin().read_line(&mut amount_str).expect("Failed to read amount");
        
        match amount_str.trim().parse::<f64>() {
            Ok(val) => return (desc.trim().to_string(), val),
            Err(_) => println!("Invalid number. Please enter a numeric value (e.g., 12.50)."),
        }
    }
}

// --- Test Suite (Requirement #3) ---

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
        tracker.add_transaction("Salary".into(), 1000.0, TransactionType::Income);
        assert_eq!(tracker.get_balance(), 1000.0);
    }

    #[test]
    fn test_add_expense() {
        let mut tracker = FinanceTracker::new();
        tracker.add_transaction("Rent".into(), 500.0, TransactionType::Expense);
        assert_eq!(tracker.get_balance(), -500.0);
    }

    #[test]
    fn test_multiple_transactions_balance() {
        let mut tracker = FinanceTracker::new();
        tracker.add_transaction("Salary".into(), 2000.0, TransactionType::Income);
        tracker.add_transaction("Food".into(), 100.0, TransactionType::Expense);
        tracker.add_transaction("Utility".into(), 50.0, TransactionType::Expense);
        assert_eq!(tracker.get_balance(), 1850.0);
    }

    #[test]
    fn test_transaction_id_incrementing() {
        let mut tracker = FinanceTracker::new();
        tracker.add_transaction("A".into(), 10.0, TransactionType::Income);
        tracker.add_transaction("B".into(), 20.0, TransactionType::Expense);
        assert_eq!(tracker.transactions[0].id, 1);
        assert_eq!(tracker.transactions[1].id, 2);
    }

    #[test]
    fn test_json_serialization_integrity() {
        let mut tracker = FinanceTracker::new();
        tracker.add_transaction("Integrity Test".into(), 99.99, TransactionType::Income);
        let json = serde_json::to_string(&tracker.transactions).expect("Failed to serialize");
        let deserialized: Vec<Transaction> = serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(deserialized[0].description, "Integrity Test");
        assert_eq!(deserialized[0].amount, 99.99);
    }
}