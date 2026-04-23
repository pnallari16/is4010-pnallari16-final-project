# is4010-pnallari16-final-project
Pranavi Nallari's Final Project for IS4010


# Personal Finance Tracker (Rust)

A robust, command-line interface (CLI) application built with **Rust** to help users manage their personal finances. This tool allows for real-time logging of income and expenses, calculates current balances, and ensures data is persisted across sessions using JSON storage.

---

## 🛠 Features
* **Real-time Balance Tracking:** Instantly see your net worth as you log transactions.
* **Data Persistence:** Transactions are saved to `transactions.json`, ensuring your data survives program restarts.
* **Input Validation:** Robust error handling prevents the program from crashing on invalid numeric inputs.
* **Automated CI/CD:** Integrated GitHub Actions workflow runs tests on every push.

---

## 🚀 Installation

### Prerequisites
* **Rust & Cargo:** Installed via [rustup.rs](https://rustup.rs/).
* **Git:** For cloning the repository.

### Setup Steps
1.  **Fork and Clone the repository:**

    * [Repository Link](https://github.com/pnallari16/is4010-pnallari16-final-project)
    * Clone into folder of your choice.

2.  **Build the project:**
    ```bash
    cargo build
    ```
3.  **Run the test suite:**
    Verify the logic with 6 built-in tests:
    ```bash
    cargo test
    ```

---

## 💻 Usage

Start the tracker by running:
```bash
cargo run
```

## Menu Options
1. **Add Income:** Log money coming in (e.g., Salary, Gifts, Bonuses).

2. **Add Expense:** Log money going out (e.g., Rent, Bills, Food).

3. **View Balance:** Displays the current net total of all transactions.

4. **List Transactions:** Shows a full history of all your logs with IDs and dates.

5. **Save and Exit:** Commits all data to `transactions.json` and closes the application.


## Realistic Examples
1. **Recording a Paycheck**
If you just recieved your monthly salary:
    1. Enter `1` at the menu
    2. **Description:** `Monthly Salary`
    3. **Amount:** `3500.00`
    4. **Result:** The program saves the data and updates your balance to **$3500.00**.

2. **Logging a Utility Bill**
If you just paid an electric bill:
    1. Enter `2` at the menu
    2. **Description:** `Electric Bill`
    3. **Amount:** `120.50`
    4. **Result:** The program subtracts the amount and updates your balance to **$3379.50**.   

3. **Expected Output (List Transactions)**
When you select option `4`, your terminal will display: 

```
--- Transaction History ---
[ID: 1] 2026-04-23 | Income: Monthly Salary - $3500.00
[ID: 2] 2026-04-23 | Expense: Electric Bill - $120.50
```

## Testing
This project includes a meaningful test suite covering:

* Initial state verification.
* Income and Expense mathematical logic.
* Balance aggregation from multiple sources.
* Transaction ID incrementing logic.
* JSON Serialization Integrity: Ensures data is correctly transformed for file storage.

## Known Limitations and Future Ideas.
* **Current Limitation:** The application does not currently support deleting or editing existing transactions.

* **Future Idea:** Add a search feature to filter transactions by date or description.

* **Future Idea:** Implement a "Categories" system (e.g., Housing, Food, Fun) for better spending analysis.