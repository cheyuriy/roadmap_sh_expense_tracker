# Expense Tracker

This project is part of the [roadmap.sh Expense Tracker project](https://roadmap.sh/projects/expense-tracker). Expense Tracker is a command-line application to manage your finances. It allows users to add, delete, and view expenses, as well as generate summaries of expenses by month or overall. The application also supports categories, spending limits, and exporting data to CSV files.
Part of

## Features

- **Add Expenses**: Add an expense with a description, amount, and optional category.
- **Delete Expenses**: Remove an expense by its ID.
- **List Expenses**: View all expenses, optionally filtered by category.
- **Summarize Expenses**: Generate a summary of expenses for a specific month or overall.
- **Manage Categories**: Add, delete, and list categories for organizing expenses.
- **Set Spending Limits**: Define a monthly spending limit and get warnings when exceeded.
- **Export to CSV**: Export all transactions to a CSV file.

## Installation
1. Ensure you have [Rust](https://www.rust-lang.org/) installed on your system.
2. Clone this repository:
    ```bash
    $ git clone https://github.com/your-username/roadmap_sh_expense_tracker.git
    ```
3. Navigate to the project directory:
    ```bash
    $ cd roadmap_sh_expense_tracker
    ```
4. Build the project:
    ```bash
    $ cargo build --release
    ```
5. Run the application:
    ```bash
    $ ./target/release/expense-tracker
    ```

## Usage

1. **Add an Expense**:
    ```bash
    $ expense-tracker add "Lunch" 20.0
    ```
    or with category ID
    ```bash
    $ expense-tracker add "Lunch" 20.0 1
    ```

2. **List Expenses:**   
    ```bash
    $ expense-tracker list
    ```

4. **Manage categories**
    ```bash
    $ expense-tracker category add Food

    $ expense-tracker category list

    $ expense-tracker category delete 1
    ```

5. **Generate Summary:**
    ```bash
    $ expense-tracker summary
    ```
    or for specific month:
    ```bash
    $ expense-tracker summary 2025-04
    ```
    or for specific category:
    ```bash
    $ expense-tracker summary 2025-04 1
    ```

6. **Set a Spending Limit:**
    ```bash
    $ expense-tracker limit 100
    ```

7. **Export to CSV:**
    ```bash
    $ expense-tracker export expenses.csv
    ```

For the full list of commands see `$ expense-tracker --help`.

## Data Storage
The application uses a JSON file (`data/data.json` by default) to persist data, ensuring all transactions and categories are saved between sessions.