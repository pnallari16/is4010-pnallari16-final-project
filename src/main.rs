use std::io::{self, Write};
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

    loop {
        println!("\n--- Finance Tracker Menu ---");
        println!("1. Add Income");
        println!("2. Add Expense");
        println!("3. View Balance");
        println!("4. List Transactions");
        println!("5. Exit");
        print!("Choose an option: ");
        io::stdout().flush().unwrap(); // Ensures the prompt appears before input

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");

        match choice.trim() {
            "1" => {
                let (desc, amount) = get_transaction_input("Income");
                my_tracker.add_transaction(desc, amount, TransactionType::Income);
                println!("Income added successfully!");
            }
            "2" => {
                let (desc, amount) = get_transaction_input("Expense");
                my_tracker.add_transaction(desc, amount, TransactionType::Expense);
                println!("Expense added successfully!");
            }
            "3" => {
                println!("\n>>> Current Balance: ${:.2}", my_tracker.get_balance());
            }
            "4" => {
                println!("\n--- Transaction History ---");
                for t in &my_tracker.transactions {
                    println!("[{}] {:?}: {} - ${:.2}", t.date, t.kind, t.description, t.amount);
                }
            }
            "5" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid option, please try again."),
        }
    }
}

// Helper function to handle user input for transactions
fn get_transaction_input(label: &str) -> (String, f64) {
    let mut desc = String::new();
    let mut amount_str = String::new();

    print!("Enter {} description: ", label);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut desc).expect("Failed to read line");

    print!("Enter amount: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut amount_str).expect("Failed to read line");

    let amount: f64 = amount_str.trim().parse().expect("Please enter a valid number");

    (desc.trim().to_string(), amount)
}