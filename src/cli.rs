use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CLI {
    #[command(subcommand)]
    pub commands: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands { 
    /// Add a new transaction
    Add {
        /// Description of the transaction
        description: String,
        /// Amount of the transaction
        amount: f64,
        /// Category of the transaction
        category: Option<String>,
    },
    /// Delete a transaction
    Delete {
        /// ID of the transaction to delete
        id: u32,
    },
    /// List all transactions
    List {
        /// Filter transactions by category
        category: Option<String>,
    },
    /// Show a summary of transactions for a given month, or overall
    Summary {
        /// Month in the format YYYY-MM, or "overall" for all transactions
        #[arg(default_value = "overall")]
        month: String,
    },
    /// Limits spending for the current month
    Limit {
        /// Amount to limit spending to
        amount: f64,
    },
    /// Export all transactions to a CSV file
    Export {
        /// Path to the output CSV file
        filename: String,
    },
    /// Manage categories
    Category {
        #[command(subcommand)]
        category_subcommand: CategorySubcommand,
    },
}

#[derive(Subcommand, Debug)]
pub enum CategorySubcommand {
    /// Add a new category
    Add {
        /// Name of the category
        name: String,
    },
    /// Delete a category
    Delete {
        /// ID of the category to delete
        id: u32,
    },
    /// List all categories
    List,
}
